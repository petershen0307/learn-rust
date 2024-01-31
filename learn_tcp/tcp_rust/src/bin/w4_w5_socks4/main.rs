use std::os::fd::AsRawFd;

use env_logger::Env;
use log::{debug, error, info};
use tcp_listener::graceful_shutdown;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
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
                    socks4_connect(shutdown_channel, socket).await
                });
            }
            _ = shutdown_receiver.recv() => {
                std::thread::sleep(std::time::Duration::from_secs(5));
                info!("server shutdown!");
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
    pub vn: u8, // socks protocol
    pub cd: u8, // command 1 be connect request
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
            dest_port: u16::from_be_bytes(data[2..4].try_into().unwrap()),
            dest_ip: u32::from_be_bytes(data[4..8].try_into().unwrap()),
            user_id: data[8..data_length].to_vec(),
        })
    }
    pub fn to_bytes_without_user_id(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.vn);
        data.push(self.cd);
        data.push(self.dest_port.to_be_bytes()[0]);
        data.push(self.dest_port.to_be_bytes()[1]);
        data.push(self.dest_ip.to_be_bytes()[0]);
        data.push(self.dest_ip.to_be_bytes()[1]);
        data.push(self.dest_ip.to_be_bytes()[2]);
        data.push(self.dest_ip.to_be_bytes()[3]);
        data
    }
}

impl std::fmt::Display for Socks4ConnectHeader {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "addr={}:{} vn={} cd={} userID={:?}",
            to_ipv4_addr(self.dest_ip),
            self.dest_port,
            self.vn,
            self.cd,
            self.user_id
        )
    }
}

fn to_ipv4_addr(ip: u32) -> std::net::Ipv4Addr {
    std::net::Ipv4Addr::new(
        ip.to_le_bytes()[3],
        ip.to_le_bytes()[2],
        ip.to_le_bytes()[1],
        ip.to_le_bytes()[0],
    )
}

#[test]
fn test_ip_convert() {
    // 127.0.0.1
    let ip = to_ipv4_addr(0x7F000001);
    assert_eq!(std::net::Ipv4Addr::new(127, 0, 0, 1), ip);
}

#[test]
fn test_socks_to_bytes_convert() {
    let expected = vec![0x4, 0x1, 0x1, 0xbb, 23, 22, 173, 247];
    let socks4 = Socks4ConnectHeader::new(expected.clone()).unwrap();
    let r = socks4.to_bytes_without_user_id();
    assert_eq!(expected, r);
}

async fn socks4_connect(
    mut shutdown_channel: tokio::sync::broadcast::Receiver<graceful_shutdown::ZeroDataType>,
    socket: tokio::net::TcpStream,
) {
    // state initial read until 0
    //let mut request_stream = tokio::io::BufStream::new(socket);
    let mut request_stream = socket;
    let mut read_buf: [u8; 1024] = [0; 1024];
    let n = request_stream.read(&mut read_buf).await.unwrap();
    let sock4_data = match Socks4ConnectHeader::new(read_buf[0..n].to_vec()) {
        Ok(n) => {
            info!("socks4 info {}", n);
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
            request_stream
                .write_all(&response.to_bytes_without_user_id())
                .await
                .unwrap();
            request_stream.shutdown().await.unwrap();
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
    request_stream
        .write_all(&response.to_bytes_without_user_id())
        .await
        .unwrap();
    if sock4_data.cd != 1 {
        request_stream.shutdown().await.unwrap();
        return;
    }
    // state successful request, wait util client close connection or server shutdown
    let dest_stream =
        tokio::net::TcpStream::connect((to_ipv4_addr(sock4_data.dest_ip), sock4_data.dest_port))
            .await;
    let mut dest_stream = match dest_stream {
        Ok(stream) => stream,
        Err(e) => {
            let error_msg = format!(
                "connect to {}:{} got error={}",
                to_ipv4_addr(sock4_data.dest_ip),
                sock4_data.dest_port,
                e
            );
            error!("{}", error_msg);
            request_stream.write(error_msg.as_bytes()).await.unwrap();
            request_stream.shutdown().await.unwrap();
            return;
        }
    };
    info!("connect to destination {} success", sock4_data);
    let mut dest_buf = [0; 1024];
    let mut request_buf = [0; 1024];
    loop {
        tokio::select! {
            // read data from dest_stream then write data to request_stream
            r = dest_stream.read(&mut dest_buf) => {
                match r {
                    Ok(n) if n == 0 => {
                        // socket closed
                        debug!("{:?} dest_stream connection close", dest_stream.peer_addr());
                        request_stream.shutdown().await.unwrap();
                        return;
                    }
                    Ok(n) => {
                        debug!("received from {:?} n={}", dest_stream.peer_addr(), n);
                        request_stream.write(&dest_buf[0..n]).await.unwrap();
                    }
                    Err(e) => {
                        error!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
            }
            // read data from request_stream then write data to dest_stream
            r = request_stream.read(&mut request_buf) => {
                match r {
                    Ok(n) if n == 0 => {
                        // socket closed
                        debug!("{} request_stream connection close", sock4_data);
                        dest_stream.shutdown().await.unwrap();
                        return;
                    }
                    Ok(n) => {
                        debug!("received from {} n={}", sock4_data, n);
                        dest_stream.write(&request_buf[0..n]).await.unwrap();
                    }
                    Err(e) => {
                        error!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
            }
            // shutdown event, close dest_stream and request_stream
            _ = shutdown_channel.recv() => {
                info!("server shutdown!");
                dest_stream.shutdown().await.unwrap();
                request_stream.shutdown().await.unwrap();
            }
        }
    }
}
