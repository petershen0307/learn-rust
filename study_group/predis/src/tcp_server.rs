pub mod command_reader;
pub mod graceful_shutdown;

use crate::{
    models::{
        configuration::Configuration,
        data_command::{self, DataWatcherMessage},
    },
    tcp_server,
};

use std::{collections::HashMap, os::fd::AsRawFd, sync::Arc};

use log::{debug, error, info};

use tokio::{io::AsyncWriteExt, net::TcpListener, sync::mpsc, sync::Semaphore};

pub async fn tcp_server(config: Configuration) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap();
    let shutdown_channel =
        graceful_shutdown::listen_sig_interrupt_to_close_socket_fd(AsRawFd::as_raw_fd(&listener));

    let (tx, mut rx) = mpsc::channel::<DataWatcherMessage>(config.workers);
    // create data watcher
    tokio::spawn(async move {
        let mut map = HashMap::<String, String>::new();
        while let Some(r) = rx.recv().await {
            let response = match r.data {
                data_command::Command::Set(s) => {
                    map.insert(s[0].clone(), s[1].clone());
                    resp::Value::String("ok".to_string())
                }
                data_command::Command::Get(g) => match map.get(&g) {
                    Some(v) => resp::Value::String(v.to_string()),
                    None => resp::Value::Null,
                },
                data_command::Command::Del(d) => match map.remove(&d) {
                    Some(_) => resp::Value::Integer(1),
                    None => resp::Value::Integer(0),
                },
                _ => resp::Value::Error("ERR unknown command".to_string()),
            };
            r.callback.send(response).unwrap();
        }
    });
    tcp_listener_handle(shutdown_channel, &listener, config.workers, tx).await;
}

async fn tcp_listener_handle(
    shutdown_channel: tokio::sync::broadcast::Sender<()>,
    listener: &TcpListener,
    concurrent_connection: usize,
    data_watcher_sender: mpsc::Sender<DataWatcherMessage>,
) {
    let semaphore = Arc::new(Semaphore::new(concurrent_connection));
    let mut shutdown_channel_main = shutdown_channel.subscribe();
    loop {
        let tx = data_watcher_sender.clone();
        tokio::select! {
            connection = listener.accept() => {
                let r = if connection.is_ok() {
                    connection.unwrap()
                }else{
                    continue
                };
                handle_connection(shutdown_channel.clone(), r, semaphore.clone(), tx).await;
            }
            _ = shutdown_channel_main.recv() => {
                info!("close listener!");
                break;
            }
        }
    }
}

async fn handle_connection(
    shutdown_channel: tokio::sync::broadcast::Sender<()>,
    connection: (tokio::net::TcpStream, std::net::SocketAddr),
    semaphore: Arc<Semaphore>,
    data_watcher_sender: mpsc::Sender<DataWatcherMessage>,
) {
    let (mut tcp_stream, addr) = connection;
    debug!("client connected={}", addr);
    let permit = semaphore.try_acquire_owned();
    if permit.is_err() {
        info!("too many connection drop this one={}", addr);
        // drop connection
        tcp_stream
            .write_all(&resp::Value::Error("ERR 429 too many connection".to_string()).encode())
            .await
            .unwrap_or_else(|x| {
                error!("write tcp stream error={}", x);
            });
        tcp_stream.shutdown().await.unwrap_or_else(|x| {
            error!("shutdown tcp stream error={}", x);
        });
        return;
    }
    let shutdown_channel = shutdown_channel.subscribe();
    let permit = permit.unwrap();
    tokio::spawn(async move {
        tcp_server::command_reader::CommandReader::new(
            shutdown_channel,
            tcp_stream,
            data_watcher_sender,
        )
        .run()
        .await;
        drop(permit);
    });
}
