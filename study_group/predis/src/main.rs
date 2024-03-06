use std::{collections::HashMap, os::fd::AsRawFd};

use predis::{
    models::{
        configuration::Configuration,
        data_command::{self, DataWatcherMessage},
    },
    tcp_server::graceful_shutdown,
};

use env_logger::Env;
use tokio::{net::TcpListener, sync::mpsc};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let config = predis::models::configuration::Configuration::new();
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

    let (tx, mut rx) = mpsc::channel::<DataWatcherMessage>(config.workers);
    // create data watcher
    tokio::spawn(async move {
        let mut map = HashMap::<String, String>::new();
        while let Some(r) = rx.recv().await {
            let response = match r.data {
                data_command::Command::Set(s) => {
                    map.insert(s[0].clone(), s[1].clone());
                    resp::Value::String("ok".to_string())
                }
                data_command::Command::Get(g) => match map.get(&g) {
                    Some(v) => resp::Value::String(v.to_string()),
                    None => resp::Value::Null,
                },
                data_command::Command::Del(d) => match map.remove(&d) {
                    Some(_) => resp::Value::Integer(1),
                    None => resp::Value::Integer(0),
                },
                _ => resp::Value::Error("ERR unknown command".to_string()),
            };
            r.callback.send(response).unwrap();
        }
    });
    predis::tcp_server::tcp_listener_handle(shutdown_channel, &listener, config.workers, tx).await;
}
