use std::{
    cell::RefCell,
    io,
    io::{Read, Write},
    net,
    net::TcpStream,
    sync::{Arc, Mutex, RwLock},
    thread, time,
};

pub fn handle_client(
    stream: RefCell<TcpStream>,
    shutdown: Arc<Mutex<bool>>,
    connection_count: Arc<Mutex<i32>>,
) {
    {
        let mut connection_count = connection_count.lock().unwrap();
        (*connection_count) += 1;
    }
    loop {
        let mut buffer = [0; 1024];
        stream.borrow_mut().set_nonblocking(true).unwrap();
        let read_size = match stream.borrow_mut().read(&mut buffer) {
            Ok(i) => {
                if i == 0 {
                    println!("read 0");
                    break;
                } else {
                    i
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                {
                    if *(shutdown.lock().unwrap()) {
                        println!(
                            "[{:?}] received shutdown event at TcpStream!",
                            thread::current().id()
                        );
                        break;
                    }
                }
                thread::sleep(time::Duration::from_millis(50));
                continue;
            }
            Err(err) => {
                println!("read err={}", err);
                break;
            }
        };
        println!(
            "[{:?}] received from ip:port={}:{}; read size={}; read message={}",
            thread::current().id(),
            stream.borrow().peer_addr().unwrap().ip(),
            stream.borrow().peer_addr().unwrap().port(),
            read_size,
            String::from_utf8_lossy(&buffer[..])
        );
        thread::sleep(time::Duration::from_secs(5));
        let response = format!(
            "HTTP/1.1 200 OK {}\r\n\r\n",
            String::from_utf8_lossy(&buffer[..])
        );
        stream.borrow_mut().write(response.as_bytes()).unwrap();
    }

    {
        if *(shutdown.lock().unwrap()) {
            stream
                .borrow_mut()
                .shutdown(net::Shutdown::Both)
                .expect("tcp stream shutdown failed!");
        }
    }
    {
        println!("[{:?}] before minus count", thread::current().id());
        let mut connection_count = connection_count.lock().unwrap();
        (*connection_count) -= 1;
        println!("[{:?}] after minus count", thread::current().id());
    }
    println!(
        "close connection {}:{}",
        stream.borrow().peer_addr().unwrap().ip(),
        stream.borrow().peer_addr().unwrap().port()
    );
}

pub async fn async_handle_client(
    stream: RefCell<TcpStream>,
    shutdown: Arc<Mutex<bool>>,
    connection_count: Arc<Mutex<i32>>,
) {
    {
        let mut connection_count = connection_count.lock().unwrap();
        (*connection_count) += 1;
    }
    loop {
        let mut buffer = [0; 1024];
        stream.borrow_mut().set_nonblocking(true).unwrap();
        let read_size = match stream.borrow_mut().read(&mut buffer) {
            Ok(i) => {
                if i == 0 {
                    println!("read 0");
                    break;
                } else {
                    i
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                {
                    if *(shutdown.lock().unwrap()) {
                        println!(
                            "[{:?}] received shutdown event at TcpStream!",
                            thread::current().id()
                        );
                        break;
                    }
                }
                thread::sleep(time::Duration::from_millis(50));
                continue;
            }
            Err(err) => {
                println!("read err={}", err);
                break;
            }
        };
        println!(
            "[{:?}] received from ip:port={}:{}; read size={}; read message={}",
            thread::current().id(),
            stream.borrow().peer_addr().unwrap().ip(),
            stream.borrow().peer_addr().unwrap().port(),
            read_size,
            String::from_utf8_lossy(&buffer[..])
        );
        thread::sleep(time::Duration::from_secs(5));
        let response = format!(
            "HTTP/1.1 200 OK {}\r\n\r\n",
            String::from_utf8_lossy(&buffer[..])
        );
        stream.borrow_mut().write(response.as_bytes()).unwrap();
    }

    {
        if *(shutdown.lock().unwrap()) {
            stream
                .borrow_mut()
                .shutdown(net::Shutdown::Both)
                .expect("tcp stream shutdown failed!");
        }
    }
    {
        println!("[{:?}] before minus count", thread::current().id());
        let mut connection_count = connection_count.lock().unwrap();
        (*connection_count) -= 1;
        println!("[{:?}] after minus count", thread::current().id());
    }
    println!(
        "close connection {}:{}",
        stream.borrow().peer_addr().unwrap().ip(),
        stream.borrow().peer_addr().unwrap().port()
    );
}

pub fn two_way_handle_client(
    stream: RefCell<TcpStream>,
    shutdown: Arc<RwLock<bool>>,
    connection_count: Arc<Mutex<i32>>,
    stdin_buffer: Arc<RwLock<String>>,
) {
    {
        let mut connection_count = connection_count.lock().unwrap();
        (*connection_count) += 1;
    }
    loop {
        let mut buffer = [0; 1024];
        stream.borrow_mut().set_nonblocking(true).unwrap();
        let read_size = match stream.borrow_mut().read(&mut buffer) {
            Ok(i) => {
                if i == 0 {
                    println!("read 0");
                    break;
                } else {
                    i
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                {
                    if *(shutdown.read().unwrap()) {
                        println!(
                            "[{:?}] received shutdown event at TcpStream!",
                            thread::current().id()
                        );
                        break;
                    }
                }
                thread::sleep(time::Duration::from_millis(50));
                0
            }
            Err(err) => {
                println!("read err={}", err);
                break;
            }
        };
        {
            let mut stdin_buffer = stdin_buffer.write().unwrap();
            if !(*stdin_buffer).is_empty() {
                let response = format!("server send message={}", stdin_buffer);
                stream.borrow_mut().write(response.as_bytes()).unwrap();
                (*stdin_buffer).clear();
            }
        }
        if read_size != 0 {
            println!(
                "[{:?}] received from ip:port={}:{}; read size={}; read message={}",
                thread::current().id(),
                stream.borrow().peer_addr().unwrap().ip(),
                stream.borrow().peer_addr().unwrap().port(),
                read_size,
                String::from_utf8_lossy(&buffer[..])
            );
            let response = format!(
                "HTTP/1.1 200 OK {}\r\n\r\n",
                String::from_utf8_lossy(&buffer[..])
            );
            stream.borrow_mut().write(response.as_bytes()).unwrap();
        }
    }

    {
        if *(shutdown.read().unwrap()) {
            stream
                .borrow_mut()
                .shutdown(net::Shutdown::Both)
                .expect("tcp stream shutdown failed!");
        }
    }
    {
        println!("[{:?}] before minus count", thread::current().id());
        let mut connection_count = connection_count.lock().unwrap();
        (*connection_count) -= 1;
        println!("[{:?}] after minus count", thread::current().id());
    }
    println!(
        "close connection {}:{}",
        stream.borrow().peer_addr().unwrap().ip(),
        stream.borrow().peer_addr().unwrap().port()
    );
}