use crate::data_watcher::execution;

use resp::Value;
use tokio::sync::oneshot;

// communicate with data watcher
pub struct DataWatcherMessage {
    pub data: Box<dyn execution::Execution + Send>,
    pub callback: oneshot::Sender<Value>,
}
