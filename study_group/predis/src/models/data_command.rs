use resp::Value as RValue;
use tokio::sync::oneshot;

#[derive(PartialEq, Debug)]
pub enum Command {
    Set(String, String),
    Get(String),
    Del(String),
    Unknown,
}

impl Command {
    pub fn to_command(cmd: &[RValue]) -> Self {
        // command value or command key value
        if cmd.len() < 2 {
            return Command::Unknown;
        }
        match cmd[0].to_string().to_lowercase().as_str() {
            "set" => {
                if cmd.len() != 3 {
                    Command::Unknown
                } else {
                    Command::Set(cmd[1].to_string(), cmd[2].to_string())
                }
            }
            "get" => Command::Get(cmd[1].to_string()),
            "del" => Command::Del(cmd[1].to_string()),
            _ => Command::Unknown,
        }
    }
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
