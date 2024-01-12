use std::{
    cell::RefCell,
    io,
    net::TcpListener,
    sync::{Arc, Mutex, RwLock},
    thread, time,
};

use tcp_listener::{graceful_shutdown, stdin};

fn main() -> std::io::Result<()> {
    let shutdown = Arc::new(RwLock::new(false));
    graceful_shutdown::listen_sig_interrupt(Arc::clone(&shutdown));

    let buffer = Arc::new(RwLock::new(String::new()));
    stdin::reading_stdin_to_buffer(Arc::clone(&buffer), Arc::clone(&shutdown));

    listen_tcp_connection(Arc::clone(&buffer), Arc::clone(&shutdown));

    Ok(())
}

fn listen_tcp_connection(buffer: Arc<RwLock<String>>, shutdown: Arc<RwLock<bool>>) {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    tcp_listener
        .set_nonblocking(true)
        .expect("set nonblocking failed");
    let connection_count = Arc::new(Mutex::new(0));
    {
        // to enforce TcpListener will be dropped after received shutdown event
        for stream in tcp_listener.incoming() {
            let shutdown_for_tcp_scream = Arc::clone(&shutdown);
            let connection_count_tcp_stream = Arc::clone(&connection_count);
            let stdin_buffer = Arc::clone(&buffer);
            match stream {
                Ok(s) => {
                    thread::spawn(move || {
                        tcp_listener::handle_client::two_way_handle_client(
                            RefCell::new(s),
                            shutdown_for_tcp_scream,
                            connection_count_tcp_stream,
                            stdin_buffer,
                        );
                    });
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    {
                        let shutdown = shutdown.read().unwrap();
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
}
