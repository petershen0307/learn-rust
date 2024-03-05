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
}
