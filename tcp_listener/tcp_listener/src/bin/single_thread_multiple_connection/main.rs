use std::{
    cell::RefCell,
    collections::HashMap,
    io,
    net::TcpListener,
    rc::Rc,
    sync::{Arc, RwLock},
    thread, time,
};

use env_logger::Env;
use log::info;

use tcp_listener::{graceful_shutdown, handle_client};
fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let shutdown = Arc::new(RwLock::new(false));
    graceful_shutdown::listen_sig_interrupt(Arc::clone(&shutdown));

    echo_listen_tcp_connection(Arc::clone(&shutdown));

    Ok(())
}

fn echo_listen_tcp_connection(shutdown: Arc<RwLock<bool>>) {
    let mut tcp_streams = HashMap::new();
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    tcp_listener
        .set_nonblocking(true)
        .expect("set nonblocking failed");
    {
        // to enforce TcpListener will be dropped after received shutdown event
        for stream in tcp_listener.incoming() {
            match stream {
                Ok(s) => {
                    tcp_streams.insert(s.peer_addr().unwrap(), Rc::new(RefCell::new(s)));
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    {
                        let shutdown = shutdown.read().unwrap();
                        if *shutdown {
                            info!("received shutdown event at TcpListener!");
                            for (_, stream) in tcp_streams.iter_mut() {
                                stream
                                    .borrow_mut()
                                    .shutdown(std::net::Shutdown::Both)
                                    .unwrap();
                            }
                            break;
                        }
                    }
                    {
                        let mut disconnected_streams = Vec::new();
                        // go through all tcp streams
                        for (address, stream) in tcp_streams.iter_mut() {
                            if handle_client::non_blocking_echo_handle_client(Rc::clone(stream)) {
                                disconnected_streams.push(address.clone());
                            }
                        }
                        // remove disconnected tcp streams
                        for address in disconnected_streams {
                            tcp_streams.remove(&address);
                        }
                    }
                    thread::sleep(time::Duration::from_millis(50));
                    continue;
                }
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }
    }
    info!("graceful shutdown!");
}
