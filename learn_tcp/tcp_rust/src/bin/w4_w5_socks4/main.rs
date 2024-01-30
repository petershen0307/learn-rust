use std::{
    os::fd::AsRawFd,
    sync::{Arc, Mutex},
};

use env_logger::Env;
use log::{debug, error, info};
use tcp_listener::graceful_shutdown;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let shutdown_channel = tcp_listener::graceful_shutdown::listen_sig_interrupt_to_close_socket_fd(
        listener.as_raw_fd(),
    );

    tcp_listener_handle(shutdown_channel.0, shutdown_channel.1, &listener).await;

    Ok(())
}

async fn echo(
    mut shutdown_channel: tokio::sync::broadcast::Receiver<graceful_shutdown::ZeroDataType>,
    mut socket: tokio::net::TcpStream,
) {
    let mut buf = [0; 1024];

    // In a loop, read data from the socket and write the data back.
    loop {
        tokio::select! {
            r = socket.read(&mut buf) => {
                let n = match r {
                    // socket closed
                    Ok(n) if n == 0 => {
                        info!("{:?} connection close", socket.peer_addr());
                        return;
                    }
                    Ok(n) => {
                        info!("received from {:?} n={}", socket.peer_addr(), n);
                        n
                    }
                    Err(e) => {
                        error!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    error!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
            _= shutdown_channel.recv() => {
                socket.shutdown().await.unwrap();
                info!("close connection {:?}", socket.peer_addr());
                return;
            }
        }
    }
}

async fn tcp_listener_handle(
    shutdown_sender: tokio::sync::broadcast::Sender<graceful_shutdown::ZeroDataType>,
    mut shutdown_receiver: tokio::sync::broadcast::Receiver<graceful_shutdown::ZeroDataType>,
    listener: &TcpListener,
) {
    loop {
        tokio::select! {
            r = listener.accept() => {
                let (socket, addr) = r.unwrap();
                let shutdown_channel=shutdown_sender.subscribe();
                tokio::spawn(async move {
                    info!("connect from {:?}", addr);
                    echo(shutdown_channel, socket).await
                });
            }
            _ = shutdown_receiver.recv() => {
                std::thread::sleep(std::time::Duration::from_millis(50));
                break;
            }
        }
    }
}
