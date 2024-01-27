use std::{
    os::fd::AsRawFd,
    sync::{Arc, Mutex},
};

use env_logger::Env;
use log::{debug, error};
use tcp_listener::graceful_shutdown;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let shutdown_channel = tcp_listener::graceful_shutdown::listen_sig_interrupt_to_close_socket_fd(
        listener.as_raw_fd(),
    );

    tcp_listener_handle(
        shutdown_channel,
        &listener,
        |c: tokio::net::TcpStream| -> Result<(), Box<dyn std::error::Error>> { Ok(()) },
    )
    .await?;

    Ok(())
}

async fn tcp_listener_handle(
    mut shutdown_channel: tokio::sync::broadcast::Receiver<graceful_shutdown::ZeroDataType>,
    listener: &TcpListener,
    connection_handle: fn(tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        tokio::select! {
            r = listener.accept() =>{
                let (mut socket, _) = r.unwrap();
                tokio::spawn(async move {
                    let mut buf = [0; 1024];

                    // In a loop, read data from the socket and write the data back.
                    loop {
                        let n = match socket.read(&mut buf).await {
                            // socket closed
                            Ok(n) if n == 0 => return,
                            Ok(n) => n,
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
                });
            }
            _ = shutdown_channel.recv() =>{
                break;
            }
        }
    }
    Ok(())
}
