use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock},
    thread, time,
};

use env_logger::Env;
use log::{debug, info};

use tcp_listener::graceful_shutdown;
fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let shutdown = Arc::new(RwLock::new(false));
    graceful_shutdown::listen_sig_interrupt(Arc::clone(&shutdown));

    chat_server(Arc::clone(&shutdown));

    Ok(())
}

fn chat_server(shutdown: Arc<RwLock<bool>>) {
    let mut tcp_streams = HashMap::new();
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut connected_client_id = 0;
    tcp_listener
        .set_nonblocking(true)
        .expect("set nonblocking failed");
    {
        // to enforce TcpListener will be dropped after received shutdown event
        for stream in tcp_listener.incoming() {
            match stream {
                Ok(s) => {
                    tcp_streams.insert(connected_client_id, s);
                    let stream = tcp_streams.get_mut(&connected_client_id).unwrap();
                    stream.set_nonblocking(true).unwrap();
                    let send_to_message = format!(
                        "greeting from sever! your are client {}\n\n",
                        connected_client_id,
                    );
                    write_to_stream(stream, &send_to_message, &connected_client_id);
                    connected_client_id += 1;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    {
                        // handle tcp listener shutdown
                        let shutdown = shutdown.read().unwrap();
                        if *shutdown {
                            info!("received shutdown event at TcpListener!");
                            for (id, stream) in tcp_streams.iter_mut() {
                                let send_to_message = format!("shutdown by server\n\n");
                                write_to_stream(stream, &send_to_message, &id);
                                stream.shutdown(std::net::Shutdown::Both).unwrap();
                            }
                            break;
                        }
                    }
                    {
                        // go through all tcp streams
                        for id in 0..connected_client_id {
                            handle_connected_client(&id, &mut tcp_streams);
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

fn handle_connected_client(current_id: &i32, tcp_streams: &mut HashMap<i32, TcpStream>) {
    let current_handle_stream = match tcp_streams.get_mut(&current_id) {
        Some(stream) => stream,
        None => {
            // skip client {id}, which could be disconnected
            return;
        }
    };
    let mut message = String::new();
    // read from client
    match BufReader::new(current_handle_stream).read_line(&mut message) {
        Ok(n) => {
            // handle client disconnect
            if n == 0 {
                info!("client {} disconnected", current_id);
                tcp_streams.remove(current_id);
                return;
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            return;
        }
        Err(e) => panic!("encountered IO error: {e}"),
    }
    // protocol
    // - [id]:message
    // - broadcast:message
    let command_message: Vec<&str> = message.split(":").collect();
    if command_message.len() != 2 {
        info!("invalid command {}", message);
        return;
    }
    let send_to_message = format!("from [{}] message={}\n\n", current_id, command_message[1]);
    debug!(
        "from [{}] command [{}] to [{}]",
        current_id, command_message[0], send_to_message
    );

    if command_message[0].eq_ignore_ascii_case("broadcast") {
        for (id, stream) in tcp_streams.iter_mut() {
            if *current_id != *id {
                write_to_stream(stream, &send_to_message, &id);
            }
        }
    } else {
        // parse command id to i32
        let target_id = match command_message[0].parse::<i32>() {
            Ok(id) => id,
            Err(_) => {
                info!("invalid command {}", command_message[0]);
                return;
            }
        };
        match tcp_streams.get_mut(&target_id) {
            Some(stream) => {
                write_to_stream(stream, &send_to_message, &target_id);
            }
            None => {
                info!("client {} not found", target_id);
                let send_to_message = format!("client {} not found\n\n", target_id);
                write_to_stream(
                    tcp_streams.get_mut(current_id).unwrap(),
                    &send_to_message,
                    &current_id,
                );
            }
        }
    }
}

fn write_to_stream(stream: &mut TcpStream, send_to_message: &String, client_id: &i32) {
    match stream.write(send_to_message.as_bytes()) {
        Ok(_) => {}
        Err(_) => {
            info!("client {} disconnected", client_id);
        }
    }
}
