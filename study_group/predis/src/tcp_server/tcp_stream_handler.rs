use crate::data_watcher::message::DataWatcherMessage;
use crate::redis_protocol::RedisProtocolAnalyzer;

use log::{debug, error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

pub struct TcpStreamHandler {
    shutdown_channel: tokio::sync::broadcast::Receiver<()>,
    tcp_stream: tokio::net::TcpStream,
    rpa: RedisProtocolAnalyzer,
}

impl TcpStreamHandler {
    pub fn new(
        shutdown_channel: tokio::sync::broadcast::Receiver<()>,
        tcp_stream: tokio::net::TcpStream,
        tx: mpsc::Sender<DataWatcherMessage>,
    ) -> Self {
        TcpStreamHandler {
            shutdown_channel,
            tcp_stream,
            rpa: RedisProtocolAnalyzer::new(tx),
        }
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                _ = self.shutdown_channel.recv() => {
                    self.tcp_stream.shutdown().await.unwrap_or_else(|x|{
                        error!("close tcp stream error={}", x);
                    });
                    info!("close tcp stream!");
                    break;
                },
                buf = Self::read_one_command(&mut self.tcp_stream) => {
                    if buf.is_empty() {
                        info!("client close={:?}", self.tcp_stream.peer_addr());
                        break;
                    }
                    debug!("input={}", std::str::from_utf8(&buf).unwrap());
                    let response = self.rpa.apply(&buf).await;
                    self.tcp_stream.write_all(&response).await.unwrap();
                }
            }
        }
    }

    // this function is protocol dependence, expected the client send the message then wait the server response
    // if the client continually sending the message, the server will be stock at this function and get out of memory
    async fn read_one_command(tcp_stream: &mut tokio::net::TcpStream) -> Vec<u8> {
        const READ_SIZE: usize = 1024;
        let mut read_buf = Vec::new();
        let mut buf = [0_u8; READ_SIZE];
        while let Ok(n) = tcp_stream.read(&mut buf).await {
            read_buf.extend_from_slice(&buf[0..n]);
            if n != READ_SIZE {
                break;
            }
        }
        read_buf
    }
}
