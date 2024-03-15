use std::collections::VecDeque;

use crate::data_watcher::execution::Execution;
use crate::data_watcher::DataStorage;

use anyhow::Result;
use resp::Value;

#[derive(Default, PartialEq, Debug)]
pub struct Del {
    key: VecDeque<String>,
}

impl Del {
    pub fn parse(input: VecDeque<String>) -> Result<Box<Self>, Value> {
        Ok(Box::new(Del { key: input }))
    }
}

impl Execution for Del {
    fn exec(&self, data: &mut DataStorage) -> Value {
        let mut result = 0;
        let mut keys = self.key.clone();
        while let Some(key) = keys.pop_front() {
            if data.remove(&key).is_some() {
                result += 1;
            }
        }
        Value::Integer(result)
    }
}
