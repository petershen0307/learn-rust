use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};

use log::info;

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
