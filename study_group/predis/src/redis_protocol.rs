use crate::models::data_command::{Command, DataWatcherMessage, RespValueExt};
use resp::Value;
use tokio::sync::{mpsc, oneshot};

pub struct RedisProtocolAnalyzer {
    query_data_channel: mpsc::Sender<DataWatcherMessage>,
}

impl RedisProtocolAnalyzer {
    pub fn new(tx: mpsc::Sender<DataWatcherMessage>) -> Self {
        RedisProtocolAnalyzer {
            query_data_channel: tx,
        }
    }
    // return the encoded server result
    pub async fn apply(&self, client_input: &[u8]) -> Vec<u8> {
        let mut resp_decoder = resp::Decoder::new(std::io::BufReader::new(client_input));
        if let Ok(Value::Array(v)) = resp_decoder.decode() {
            match Self::parse(v) {
                Ok(cmd) => {
                    let (callback_tx, callback_rx) = oneshot::channel();
                    let msg = DataWatcherMessage {
                        data: cmd,
                        callback: callback_tx,
                    };
                    let _ = self.query_data_channel.send(msg).await;
                    match callback_rx.await {
                        Ok(v) => v.encode(),
                        Err(_) => Value::Error("get data failed".to_string()).encode(),
                    }
                }
                Err(e) => e.encode(),
            }
        } else {
            Value::Error("decode error".to_string()).encode()
        }
    }

    // parse resp array to command and value
    fn parse(cmd: Vec<Value>) -> Result<Command, Value> {
        // command value or command key value
        if cmd.len() < 2 {
            return Result::Err(Value::Error("command parse error".to_string()));
        }
        match cmd[0].to_string().to_lowercase().as_str() {
            "set" => {
                if cmd.len() != 3 {
                    Result::Err(Value::Error("command parse error".to_string()))
                } else {
                    Ok(Command::Set(cmd[1].to_string(), cmd[2].to_string()))
                }
            }
            "get" => Ok(Command::Get(cmd[1].to_string())),
            "del" => Ok(Command::Del(cmd[1].to_string())),
            _ => Result::Err(Value::Error(format!(
                "command {} not support",
                cmd[0].to_string()
            ))),
        }
    }
}

#[tokio::test]
async fn test_apply_set_command() {
    // arrange
    //*3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n
    let input_value = Value::Array(vec![
        Value::Bulk("set".to_string()),
        Value::Bulk("key".to_string()),
        Value::Bulk("value".to_string()),
    ]);
    let (tx, mut rx) = mpsc::channel::<DataWatcherMessage>(1);
    let rpa = RedisProtocolAnalyzer {
        query_data_channel: tx,
    };
    // mock data watcher
    tokio::spawn(async move {
        let data = rx.recv().await.unwrap();
        assert!(data.callback.send(Value::String("ok".to_string())).is_ok())
    });
    // act
    println!("{:?}", String::from_utf8(input_value.encode()));
    let r = rpa.apply(&input_value.encode()).await;
    // assert
    assert_eq!(Value::String("ok".to_string()).encode(), r);
}

#[test]
fn test_parse_command_set_key_with_value_string() {
    // arrange
    let input_value = vec![
        Value::Bulk("set".to_string()),
        Value::Bulk("key".to_string()),
        Value::Bulk("value".to_string()),
    ];
    // act
    let r = RedisProtocolAnalyzer::parse(input_value);
    // assert
    assert!(r.is_ok());
    let r = r.unwrap();
    assert_eq!(Command::Set("key".to_string(), "value".to_string()), r);
}

#[test]
fn test_parse_command_get_key() {
    // arrange
    let input_value = vec![
        Value::Bulk("get".to_string()),
        Value::Bulk("key".to_string()),
    ];
    // act
    let r = RedisProtocolAnalyzer::parse(input_value);
    // assert
    assert!(r.is_ok());
    let r = r.unwrap();
    assert_eq!(Command::Get("key".to_string()), r);
}

#[test]
fn test_parse_command_del_key() {
    // arrange
    let input_value = vec![
        Value::Bulk("del".to_string()),
        Value::Bulk("key".to_string()),
    ];
    // act
    let r = RedisProtocolAnalyzer::parse(input_value);
    // assert
    assert!(r.is_ok());
    let r = r.unwrap();
    assert_eq!(Command::Del("key".to_string()), r);
}
