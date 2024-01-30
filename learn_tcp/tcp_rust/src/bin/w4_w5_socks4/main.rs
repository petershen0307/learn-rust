use std::{
    os::fd::AsRawFd,
    rc::Rc,
    sync::{Arc, Mutex},
};

use env_logger::Env;
use log::{debug, error, info};
use tcp_listener::graceful_shutdown;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let shutdown_channel = tcp_listener::graceful_shutdown::listen_sig_interrupt_to_close_socket_fd(
        listener.as_raw_fd(),
    );

    tcp_listener_handle(shutdown_channel.0, shutdown_channel.1, &listener).await;

    Ok(())
}

async fn echo(
    mut shutdown_channel: tokio::sync::broadcast::Receiver<graceful_shutdown::ZeroDataType>,
    mut socket: tokio::net::TcpStream,
) {
    let mut buf = [0; 1024];

    // In a loop, read data from the socket and write the data back.
    loop {
        tokio::select! {
            r = socket.read(&mut buf) => {
                let n = match r {
                    // socket closed
                    Ok(n) if n == 0 => {
                        info!("{:?} connection close", socket.peer_addr());
                        return;
                    }
                    Ok(n) => {
                        info!("received from {:?} n={}", socket.peer_addr(), n);
                        n
                    }
                    Err(e) => {
                        error!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    error!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
            _= shutdown_channel.recv() => {
                socket.shutdown().await.unwrap();
                info!("close connection {:?}", socket.peer_addr());
                return;
            }
        }
    }
}

async fn tcp_listener_handle(
    shutdown_sender: tokio::sync::broadcast::Sender<graceful_shutdown::ZeroDataType>,
    mut shutdown_receiver: tokio::sync::broadcast::Receiver<graceful_shutdown::ZeroDataType>,
    listener: &TcpListener,
) {
    loop {
        tokio::select! {
            r = listener.accept() => {
                let (socket, addr) = r.unwrap();
                let shutdown_channel=shutdown_sender.subscribe();
                tokio::spawn(async move {
                    info!("connect from {:?}", addr);
                    echo(shutdown_channel, socket).await
                });
            }
            _ = shutdown_receiver.recv() => {
                std::thread::sleep(std::time::Duration::from_millis(50));
                break;
            }
        }
    }
}

#[derive(Clone)]
struct Socks4ConnectHeader {
    // https://www.openssh.com/txt/socks4.protocol
    // request data format
    //              +----+----+----+----+----+----+----+----+----+----+....+----+
    //              | VN | CD | DSTPORT |      DSTIP        | USERID       |NULL|
    //              +----+----+----+----+----+----+----+----+----+----+....+----+
    // # of bytes:	   1    1      2              4           variable       1
    // VN is the SOCKS protocol version number and should be 4. CD is the
    // SOCKS command code and should be 1 for CONNECT request. NULL is a byte
    // of all zero bits.
    // response data format
    //              +----+----+----+----+----+----+----+----+
    //              | VN | CD | DSTPORT |      DSTIP        |
    //              +----+----+----+----+----+----+----+----+
    // # of bytes:	   1    1      2              4
    // VN is the version of the reply code and should be 0. CD is the result
    // code with one of the following values:
    // 90: request granted
    // 91: request rejected or failed
    // 92: request rejected because SOCKS server cannot connect to
    //     identd on the client
    // 93: request rejected because the client program and identd
    //     report different user-ids
    // The remaining fields are ignored.
    pub vn: u8,
    pub cd: u8,
    pub dest_port: u16,
    pub dest_ip: u32,
    pub user_id: Vec<u8>,
}

impl Socks4ConnectHeader {
    pub fn new(data: Vec<u8>) -> std::result::Result<Self, String> {
        let data_length = data.len();
        if data_length < 8 {
            error!("invalid request");
            return Err(format!("invalid request length={}", data_length));
        }
        if data[0] != 4 {
            error!("vn is not 4 ({})", data[0]);
            return Err(format!("vn is not 4 ({})", data[0]));
        }
        Ok(Socks4ConnectHeader {
            vn: data[0],
            cd: data[1],
            dest_port: u16::from_ne_bytes(data[2..4].try_into().unwrap()),
            dest_ip: u32::from_ne_bytes(data[4..8].try_into().unwrap()),
            user_id: data[8..data_length].to_vec(),
        })
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.vn);
        data.push(self.cd);
        data.push(self.dest_port.to_le_bytes()[0]);
        data.push(self.dest_port.to_le_bytes()[1]);
        data.push(self.dest_ip.to_le_bytes()[0]);
        data.push(self.dest_ip.to_le_bytes()[1]);
        data.push(self.dest_ip.to_le_bytes()[2]);
        data.push(self.dest_ip.to_le_bytes()[3]);
        data
    }
}

async fn socks4_connect(
    mut shutdown_channel: tokio::sync::broadcast::Receiver<graceful_shutdown::ZeroDataType>,
    mut socket: tokio::net::TcpStream,
) {
    // state initial read until 0
    let mut buf_stream = tokio::io::BufStream::new(socket);
    let mut read_buf = Vec::new();
    buf_stream.read_until(0, &mut read_buf).await.unwrap();
    let sock4_data = match Socks4ConnectHeader::new(read_buf) {
        Ok(n) => {
            info!("vn: {}", n.vn);
            info!("cd: {}", n.cd);
            info!("port: {}", n.dest_port);
            info!("ip: {}", n.dest_ip);
            n
        }
        Err(e) => {
            // state failed request, response then close the connection
            error!("got request data format error={}, close connection", e);
            let response = Socks4ConnectHeader {
                vn: 0,
                cd: 91,
                dest_port: 0,
                dest_ip: 0,
                user_id: Vec::new(),
            };
            buf_stream.write_all(&response.to_bytes()).await.unwrap();
            buf_stream.shutdown().await.unwrap();
            return;
        }
    };
    let mut response = sock4_data.clone();
    response.vn = 0;
    response.cd = if sock4_data.cd == 1 {
        90
    } else {
        error!("not connect protocol={}", sock4_data.cd);
        91
    };
    response.user_id.clear();
    buf_stream.write_all(&response.to_bytes()).await.unwrap();
    if sock4_data.cd != 1 {
        buf_stream.shutdown().await.unwrap();
        return;
    }
    // state successful request, wait util client close connection or server shutdown
}
