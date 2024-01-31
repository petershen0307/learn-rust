use std::{
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};

use log::{debug, error, info};
use signal_hook::{consts::SIGINT, iterator::Signals};

pub fn listen_sig_interrupt(shutdown: Arc<RwLock<bool>>) -> JoinHandle<()> {
    let mut signals = Signals::new(&[SIGINT]).unwrap();
    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGINT => {
                    let mut shutdown = shutdown.write().unwrap();
                    *shutdown = true;
                    break;
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
    let mut signals = Signals::new(&[SIGINT]).unwrap();
    let (sender, receiver) = tokio::sync::broadcast::channel(1);
    let sender_clone = sender.clone();
    tokio::spawn(async move {
        for sig in signals.forever() {
            match sig {
                SIGINT => {
                    debug!("received interrupt signal");
                    unsafe {
                        match libc::close(socket_fd) {
                            0 => (),
                            ret => error!("close tcp listener error={}", ret),
                        }
                    }
                    let none = ZeroDataType {};
                    sender_clone.send(none).unwrap();
                    break;
                }
                _ => unreachable!(),
            }
        }
        info!("[{:?}] leave signal thread!", thread::current().id());
    });
    (sender, receiver)
}
