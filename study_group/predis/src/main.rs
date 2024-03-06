use env_logger::Env;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let config = predis::models::configuration::Configuration::new();
    predis::tcp_server::tcp_server(config).await;
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
