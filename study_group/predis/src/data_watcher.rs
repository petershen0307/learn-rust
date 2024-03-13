pub mod message;
use std::collections::HashMap;

use crate::data_watcher::message::DataWatcherMessage;
use crate::redis_protocol::command::Command;

pub async fn new(mut rx: tokio::sync::mpsc::Receiver<DataWatcherMessage>) {
    // create data watcher
    tokio::spawn(async move {
        let mut map = HashMap::<String, String>::new();
        while let Some(r) = rx.recv().await {
            let response = match r.data {
                Command::Set(k, v) => {
                    map.insert(k, v);
                    resp::Value::String("ok".to_string())
                }
                Command::Get(g) => match map.get(&g) {
                    Some(v) => resp::Value::String(v.to_string()),
                    None => resp::Value::Null,
                },
                Command::Del(d) => match map.remove(&d) {
                    Some(_) => resp::Value::Integer(1),
                    None => resp::Value::Integer(0),
                },
            };
            r.callback.send(response).unwrap();
        }
    });
}
