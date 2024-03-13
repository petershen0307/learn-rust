use std::os::fd::AsRawFd;

use predis::{
    configuration::Configuration,
    data_watcher::{self, message::DataWatcherMessage},
    tcp_server::graceful_shutdown,
};

use env_logger::Env;
use tokio::{net::TcpListener, sync::mpsc};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let config = Configuration::new();
    predis_server(config).await;
    // tcp server
    // import library https://docs.rs/resp/latest/resp/struct.Decoder.html
    // make a hash map
    // support command set, get, del

    // refactor

    // tcp server
    // only handle tcp_listen(accept) and tcp_stream(read/write bytes)
    // rate limit
    // use actor pattern with a callback one shot channel

    // redis protocol analyzer
    // first convert bytes to resp::Value
    // second convert resp::Value to Command data structure
    // final send message(DataWatcherMessage) to data watcher to operate the message (actor pattern)

    // data watcher
    // receive message(DataWatcherMessage) and operate
    // response message with resp::Value
}

async fn predis_server(config: Configuration) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap();
    let shutdown_channel =
        graceful_shutdown::listen_sig_interrupt_to_close_socket_fd(AsRawFd::as_raw_fd(&listener));

    let (tx, rx) = mpsc::channel::<DataWatcherMessage>(config.workers);

    data_watcher::new(rx).await;

    predis::tcp_server::tcp_listener_handle(shutdown_channel, &listener, config.workers, tx).await;
}
