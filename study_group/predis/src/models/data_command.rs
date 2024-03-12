use resp::Value as RValue;
use tokio::sync::oneshot;

#[derive(PartialEq, Debug)]
pub enum Command {
    Set(String, String),
    Get(String),
    Del(String),
}

pub trait RespValueExt {
    fn to_string(&self) -> String;
}

impl RespValueExt for RValue {
    fn to_string(&self) -> String {
        match self {
            RValue::String(s) => s.clone(),
            RValue::Bulk(s) => s.clone(),
            _ => String::new(),
        }
    }
}

// communicate with data watcher
pub struct DataWatcherMessage {
    pub data: Command,
    pub callback: oneshot::Sender<RValue>,
}
