use std::io::{self, Write};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

use log::info;

pub fn reading_stdin_to_buffer(
    buffer: Arc<RwLock<String>>,
    shutdown: Arc<RwLock<bool>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let shutdown = shutdown.read().unwrap();
            if *shutdown {
                info!(
                    "[{:?}] received shutdown event at read thread!",
                    thread::current().id()
                );
                break;
            }
            let mut read_buffer = String::new();
            io::stdout().lock().write_all(b"please input: ").unwrap();
            io::stdout().lock().flush().unwrap();
            io::BufRead::read_line(&mut io::stdin().lock(), &mut read_buffer).unwrap();
            {
                let mut buffer = buffer.write().unwrap();
                buffer.clear();
                (*buffer).push_str(&read_buffer);
            }
        }
        info!("[{:?}] leave read thread!", thread::current().id())
    })
}
