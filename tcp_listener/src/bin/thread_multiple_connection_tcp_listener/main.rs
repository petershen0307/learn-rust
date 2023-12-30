use std::{cell::RefCell, net::TcpListener};

fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in tcp_listener.incoming() {
        std::thread::spawn(|| {
            tcp_listener::handle_client::handle_client(RefCell::new(stream.unwrap()));
        });
    }
    Ok(())
}
