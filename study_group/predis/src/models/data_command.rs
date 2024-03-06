use resp::Value as RValue;
use tokio::sync::oneshot;

#[derive(PartialEq, Debug)]
pub enum Command {
    Set(Vec<String>),
    Get(String),
    Del(String),
    Unknown,
}

impl Command {
    pub fn to_command(cmd: &[RValue]) -> Self {
        let str_cmd: Vec<String> = cmd
            .iter()
            .map(|x| match x {
                RValue::Bulk(s) => s.clone(),
                _ => String::default(),
            })
            .collect();
        match str_cmd[0].to_ascii_lowercase().as_str() {
            "set" => Command::Set(str_cmd[1..].to_vec()),
            "get" => Command::Get(str_cmd[1].clone()),
            "del" => Command::Del(str_cmd[1].clone()),
            _ => Command::Unknown,
        }
    }
}

// communicate with data watcher
pub struct DataWatcherMessage {
    pub data: Command,
    pub callback: oneshot::Sender<resp::Value>,
}
