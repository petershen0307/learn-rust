use std::{
    cell::RefCell,
    io,
    net::TcpListener,
    sync::{Arc, Mutex, RwLock},
    thread, time,
};

use env_logger::Env;
use log::info;

use tcp_listener::{graceful_shutdown, spin_wait_group::WaitGroup, stdin};
fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let shutdown = Arc::new(RwLock::new(false));
    graceful_shutdown::listen_sig_interrupt(Arc::clone(&shutdown));

    let buffer = Arc::new(RwLock::new(String::new()));
    stdin::reading_stdin_to_buffer(Arc::clone(&buffer), Arc::clone(&shutdown));

    listen_tcp_connection(Arc::clone(&buffer), Arc::clone(&shutdown));

    Ok(())
}

fn listen_tcp_connection(buffer: Arc<RwLock<String>>, shutdown: Arc<RwLock<bool>>) {
    let wg = Arc::new(Mutex::new(WaitGroup::new()));
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    tcp_listener
        .set_nonblocking(true)
        .expect("set nonblocking failed");
    {
        // to enforce TcpListener will be dropped after received shutdown event
        for stream in tcp_listener.incoming() {
            match stream {
                Ok(s) => {
                    let shutdown_for_tcp_scream = Arc::clone(&shutdown);
                    let wg_clone = Arc::clone(&wg);
                    let stdin_buffer = Arc::clone(&buffer);
                    wg_clone.lock().unwrap().add(1);
                    thread::spawn(move || {
                        tcp_listener::handle_client::two_way_handle_client(
                            RefCell::new(s),
                            shutdown_for_tcp_scream,
                            wg_clone,
                            stdin_buffer,
                        );
                    });
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    {
                        let shutdown = shutdown.read().unwrap();
                        if *shutdown {
                            info!("received shutdown event at TcpListener!");
                            break;
                        }
                    }
                    thread::sleep(time::Duration::from_millis(50));
                    continue;
                }
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }
    }
    WaitGroup::spin_wait(wg);
    info!("graceful shutdown!");
}
