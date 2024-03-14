pub mod execution;
pub mod message;

use std::{
    collections::HashMap,
    time::{self, UNIX_EPOCH},
};

use crate::data_watcher::message::DataWatcherMessage;

pub type DataStorage = HashMap<String, DataTTL>;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct DataTTL {
    value: String,
    expired_epoch: Option<time::Duration>,
}

impl DataTTL {
    pub fn new(value: String) -> Self {
        DataTTL {
            value,
            ..Default::default()
        }
    }

    pub fn update(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub fn ttl(mut self, ttl: &time::Duration) -> Self {
        self.expired_epoch = Some(
            time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                + *ttl,
        );
        self
    }

    pub fn expired(mut self, expired: &time::Duration) -> Self {
        self.expired_epoch = Some(*expired);
        self
    }

    pub fn get(&self) -> Option<String> {
        if let Some(expired) = self.expired_epoch {
            if time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                > expired
            {
                return None;
            }
        }
        Some(self.value.to_owned())
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
