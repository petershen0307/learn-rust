use std::io::{Read, Write};
use std::{
    cell::RefCell,
    net::{TcpListener, TcpStream},
    time::Duration,
};

fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in tcp_listener.incoming() {
        handle_client(RefCell::new(stream?));
    }
    Ok(())
}

fn handle_client(stream: RefCell<TcpStream>) {
    println!(
        "received from ip={}",
        stream.borrow().peer_addr().unwrap().ip().to_string()
    );
    loop{
        let mut buffer = [0; 1024];
        let read_size = stream.borrow_mut().read(&mut buffer).unwrap();
        println!(
            "read size={}; read message={}",
            read_size,
            String::from_utf8_lossy(&buffer[..])
        );
        std::thread::sleep(Duration::from_secs(5));
        let response = format!("HTTP/1.1 200 OK {}\r\n\r\n", String::from_utf8_lossy(&buffer[..]));
        stream.borrow_mut().write(response.as_bytes()).unwrap();
    }
}
