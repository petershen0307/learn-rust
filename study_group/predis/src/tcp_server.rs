pub mod graceful_shutdown;
pub mod tcp_stream_handler;

use crate::{models::data_command::DataWatcherMessage, tcp_server};

use std::sync::Arc;

use log::{debug, error, info};

use tokio::{io::AsyncWriteExt, net::TcpListener, sync::mpsc, sync::Semaphore};

pub async fn tcp_listener_handle(
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
        tcp_server::tcp_stream_handler::TcpStreamHandler::new(
            shutdown_channel,
            tcp_stream,
            data_watcher_sender,
        )
        .run()
        .await;
        drop(permit);
    });
}
