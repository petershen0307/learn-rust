use std::{
    cell::RefCell,
    io,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread, time,
};

use env_logger::Env;
use signal_hook::{consts::SIGINT, iterator::Signals};

fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let shutdown = Arc::new(Mutex::new(false));
    let shutdown_cloned = Arc::clone(&shutdown);
    let mut signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGINT => {
                    let mut shutdown = shutdown.lock().unwrap();
                    *shutdown = true;
                    break;
                }
                _ => unreachable!(),
            }
        }
    });
    let connection_count = Arc::new(Mutex::new(0));
    {
        // to enforce TcpListener will be dropped after received shutdown event
        let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        tcp_listener
            .set_nonblocking(true)
            .expect("set nonblocking failed");
        for stream in tcp_listener.incoming() {
            match stream {
                Ok(s) => {
                    // this is single thread, so main thread will be block at tcp_listener::handle_client::handle_client()
                    // In this pattern, when program receive a shutdown event, TcpListener will still finish all incoming connection then go shutdown.
                    tcp_listener::handle_client::handle_client(
                        RefCell::new(s),
                        Arc::clone(&shutdown_cloned),
                        Arc::clone(&connection_count),
                    );
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    {
                        let shutdown = shutdown_cloned.lock().unwrap();
                        if *shutdown {
                            println!("received shutdown event at TcpListener!");
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
    println!("graceful shutdown!");
    loop {
        let go_to_sleep: bool;
        {
            let connection_count = connection_count.lock().unwrap();
            go_to_sleep = (*connection_count) != 0
        }
        if go_to_sleep {
            thread::sleep(time::Duration::from_millis(50));
        } else {
            break;
        }
    }
    Ok(())
}
