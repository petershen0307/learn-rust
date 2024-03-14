pub mod execution;
pub mod message;

use std::{collections::HashMap, time};

use crate::data_watcher::message::DataWatcherMessage;

pub type DataStorage = HashMap<String, DataTTL>;

pub struct DataTTL {
    value: String,
    ttl: time::Duration,
    expired: time::SystemTime,
}

impl DataTTL {
    pub fn new(value: String, ttl: time::Duration) -> Self {
        DataTTL {
            value,
            ttl,
            expired: time::SystemTime::now() + ttl,
        }
    }
    pub fn get(&self) -> Option<String> {
        if self.ttl != time::Duration::default() && time::SystemTime::now() > self.expired {
            None
        } else {
            Some(self.value.to_owned())
        }
    }
}

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
