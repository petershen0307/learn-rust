use std::io::{Read, Write};
use std::{cell::RefCell, net::TcpStream, time::Duration};

pub fn handle_client(stream: RefCell<TcpStream>) {
    println!(
        "received from ip:port={}:{}",
        stream.borrow().peer_addr().unwrap().ip(),
        stream.borrow().peer_addr().unwrap().port()
    );
    loop {
        let mut buffer = [0; 1024];
        let read_size = match stream.borrow_mut().read(&mut buffer) {
            Ok(i) => {
                if i == 0 {
                    println!("read 0");
                    break;
                } else {
                    i
                }
            }
            Err(err) => {
                println!("read err={}", err);
                break;
            }
        };
        println!(
            "read size={}; read message={}",
            read_size,
            String::from_utf8_lossy(&buffer[..])
        );
        std::thread::sleep(Duration::from_secs(5));
        let response = format!(
            "HTTP/1.1 200 OK {}\r\n\r\n",
            String::from_utf8_lossy(&buffer[..])
        );
        stream.borrow_mut().write(response.as_bytes()).unwrap();
    }
    println!(
        "close connection {}:{}",
        stream.borrow().peer_addr().unwrap().ip(),
        stream.borrow().peer_addr().unwrap().port()
    );
}
