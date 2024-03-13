pub mod execution;
pub mod message;

use std::collections::HashMap;

use crate::data_watcher::message::DataWatcherMessage;

pub type DataStorage = HashMap<String, String>;

pub async fn new(mut rx: tokio::sync::mpsc::Receiver<DataWatcherMessage>) {
    // create data watcher
    tokio::spawn(async move {
        let mut map = DataStorage::new();
        while let Some(r) = rx.recv().await {
            let response = r.data.exec(&mut map);
            r.callback.send(response).unwrap();
        }
    });
}
