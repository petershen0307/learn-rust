pub mod command_reader;
pub mod graceful_shutdown;

use crate::{models::configuration::Configuration, tcp_server};

use std::{os::fd::AsRawFd, sync::Arc};

use log::{debug, error, info};
use tokio::{io::AsyncWriteExt, net::TcpListener, sync::Semaphore};

pub async fn tcp_server(config: Configuration) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap();
    let shutdown_channel =
        graceful_shutdown::listen_sig_interrupt_to_close_socket_fd(AsRawFd::as_raw_fd(&listener));

    tcp_listener_handle(shutdown_channel, &listener, config.workers).await;
}

async fn tcp_listener_handle(
    shutdown_channel: tokio::sync::broadcast::Sender<()>,
    listener: &TcpListener,
    concurrent_connection: usize,
) {
    let semaphore = Arc::new(Semaphore::new(concurrent_connection));
    let mut shutdown_channel_main = shutdown_channel.subscribe();
    loop {
        tokio::select! {
            connection = listener.accept() => {
                let r = if connection.is_ok() {
                    connection.unwrap()
                }else{
                    continue
                };
                handle_connection(shutdown_channel.clone(), r, semaphore.clone()).await;
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
) {
    let (mut tcp_stream, addr) = connection;
    debug!("client connected={}", addr);
    let permit = semaphore.try_acquire_owned();
    if permit.is_err() {
        info!("too many connection drop this one={}", addr);
        // drop connection
        tcp_stream
            .write_all(b"-ERR 429 too many connection\r\n")
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
        tcp_server::command_reader::CommandReader::new(shutdown_channel, tcp_stream)
            .run()
            .await;
        drop(permit);
    });
}
