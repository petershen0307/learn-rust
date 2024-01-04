use std::{
    cell::RefCell,
    io,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread, time,
};

use signal_hook::{consts::SIGINT, iterator::Signals};

fn main() -> std::io::Result<()> {
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
        println!("[{:?}] leave signal thread!", thread::current().id())
    });
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    tcp_listener
        .set_nonblocking(true)
        .expect("set nonblocking failed");
    let connection_count = Arc::new(Mutex::new(0));
    {
        // to enforce TcpListener will be dropped after received shutdown event
        for stream in tcp_listener.incoming() {
            let shutdown_for_tcp_scream = Arc::clone(&shutdown_cloned);
            let connection_count_tcp_stream = Arc::clone(&connection_count);
            match stream {
                Ok(s) => {
                    thread::spawn(move || {
                        tcp_listener::handle_client::handle_client(
                            RefCell::new(s),
                            shutdown_for_tcp_scream,
                            connection_count_tcp_stream,
                        );
                    });
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
    {
        let connection_count = connection_count.lock().unwrap();
        println!("current connection count={}", (*connection_count));
    }
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
