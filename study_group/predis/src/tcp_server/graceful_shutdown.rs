use std::thread;

use log::{debug, error, info};
use tokio::{signal, sync::broadcast::error::SendError};

// zero size data type: https://doc.rust-lang.org/nomicon/exotic-sizes.html
pub fn listen_sig_interrupt_to_close_socket_fd(
    socket_fd: i32,
) -> tokio::sync::broadcast::Sender<()> {
    let (sender, _) = tokio::sync::broadcast::channel(1);
    let sender_clone = sender.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for event");
        info!("received interrupt signal");
        unsafe {
            match libc::close(socket_fd) {
                0 => (),
                ret => error!("close tcp listener error={}", ret),
            }
        }
        sender_clone.send(()).unwrap_or_else(|x: SendError<()>| {
            error!("shutdown broadcast err={}", x);
            0
        });
        info!("[{:?}] leave signal thread!", thread::current().id());
    });
    sender
}
