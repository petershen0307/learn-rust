use std::{
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};

use log::{debug, error, info};
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};

pub fn listen_sig_interrupt(shutdown: Arc<RwLock<bool>>) -> JoinHandle<()> {
    let mut signals = Signals::new([SIGINT, SIGTERM]).unwrap();
    thread::spawn(move || {
        if let Some(sig) = signals.forever().next() {
            match sig {
                SIGINT | SIGTERM => {
                    let mut shutdown = shutdown.write().unwrap();
                    *shutdown = true;
                }
                _ => unreachable!(),
            }
        }
        info!("[{:?}] leave signal thread!", thread::current().id())
    })
}

#[derive(Debug, Clone)]
pub struct ZeroDataType {}

// this function will block the caller
pub fn listen_sig_interrupt_to_close_socket_fd(
    socket_fd: i32,
) -> (
    tokio::sync::broadcast::Sender<ZeroDataType>,
    tokio::sync::broadcast::Receiver<ZeroDataType>,
) {
    let mut signals = Signals::new([SIGINT, SIGTERM]).unwrap();
    let (sender, receiver) = tokio::sync::broadcast::channel(1);
    let sender_clone = sender.clone();
    tokio::spawn(async move {
        if let Some(sig) = signals.forever().next() {
            match sig {
                SIGINT | SIGTERM => {
                    debug!("received interrupt signal");
                    unsafe {
                        match libc::close(socket_fd) {
                            0 => (),
                            ret => error!("close tcp listener error={}", ret),
                        }
                    }
                    let none = ZeroDataType {};
                    sender_clone.send(none).unwrap();
                }
                _ => unreachable!(),
            }
        }
        info!("[{:?}] leave signal thread!", thread::current().id());
    });
    (sender, receiver)
}
