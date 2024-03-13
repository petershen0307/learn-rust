pub mod message;
use std::collections::HashMap;
use std::io::BufReader;

use crate::data_watcher::message::DataWatcherMessage;
use crate::redis_protocol::command::{self, Command};

use resp::Value;

pub async fn new(mut rx: tokio::sync::mpsc::Receiver<DataWatcherMessage>) {
    // create data watcher
    tokio::spawn(async move {
        let mut map = HashMap::<String, String>::new();
        while let Some(r) = rx.recv().await {
            let response = match r.data {
                Command::Set(k, v) => {
                    map.insert(k, v);
                    Value::String("ok".to_string())
                }
                Command::Get(g) => match map.get(&g) {
                    Some(v) => Value::String(v.to_string()),
                    None => Value::Null,
                },
                Command::Del(d) => match map.remove(&d) {
                    Some(_) => Value::Integer(1),
                    None => Value::Integer(0),
                },
                Command::Cmd => {
                    let result = [
                        command::CMD_DOCS_SET,
                        command::CMD_DOCS_GET,
                        command::CMD_DOCS_DEL,
                    ];
                    let r = result.map(|x| {
                        resp::Decoder::new(BufReader::new(x.as_bytes()))
                            .decode()
                            .unwrap()
                    });
                    let mut output = Vec::new();
                    for i in r {
                        if let Value::Array(mut v) = i {
                            output.append(&mut v);
                        }
                    }
                    Value::Array(output)
                }
            };
            r.callback.send(response).unwrap();
        }
    });
}
