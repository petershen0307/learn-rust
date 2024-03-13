use crate::redis_protocol::command::Command;

use resp::Value;
use tokio::sync::oneshot;

// communicate with data watcher
pub struct DataWatcherMessage {
    pub data: Command,
    pub callback: oneshot::Sender<Value>,
}
