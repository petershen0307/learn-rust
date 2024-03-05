use tokio::net::TcpStream;

pub enum Command {
    Data(TcpStream),
}
