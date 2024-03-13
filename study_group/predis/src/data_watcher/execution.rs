use crate::data_watcher::DataStorage;

use resp::Value;

pub trait Execution {
    fn exec(&self, data: &mut DataStorage) -> Value;
}
