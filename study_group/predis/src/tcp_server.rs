pub mod command_reader;
pub mod graceful_shutdown;

use crate::models::{command::Command, configuration::Configuration};

use std::os::fd::AsRawFd;

use log::{debug, error, info};
use tokio::net::TcpListener;

pub async fn tcp_server(config: Configuration) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap();
    let shutdown_channel =
        graceful_shutdown::listen_sig_interrupt_to_close_socket_fd(AsRawFd::as_raw_fd(&listener));

    let (msg_tx, msg_rx) = async_channel::bounded(config.workers as usize);
    let mut workers = Vec::new();
    for _ in 0..config.workers {
        let shutdown_channel = shutdown_channel.subscribe();
        let msg_rx = msg_rx.clone();
        workers.push(tokio::spawn(async move {
            command_reader::command_reader(shutdown_channel, msg_rx).await
        }));
    }
    let shutdown_channel = shutdown_channel.subscribe();
    tcp_listener_handle(shutdown_channel, &listener, msg_tx).await;
    while let Some(worker) = workers.pop() {
        tokio::join!(worker)
            .0
            .unwrap_or_else(|x| error!("join error={}", x));
    }
}

async fn tcp_listener_handle(
    mut shutdown_channel: tokio::sync::broadcast::Receiver<()>,
    listener: &TcpListener,
    channel: async_channel::Sender<Command>,
) {
    loop {
        tokio::select! {
            r = listener.accept() => {
                let (socket, addr) = r.unwrap();
                debug!("client connected={}", addr);
                channel.send(Command::Data(socket)).await.unwrap_or_else(|x|{
                    error!("send command error={}", x);
                });
            }
            _ = shutdown_channel.recv() => {
                info!("close listener!");
                break;
            }
        }
    }
}
