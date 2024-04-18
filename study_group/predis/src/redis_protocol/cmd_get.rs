use std::collections::VecDeque;

use crate::data_watcher::execution::Execution;
use crate::data_watcher::DataStorage;

use anyhow::Result;
use resp::Value;

#[derive(Default, PartialEq, Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn parse(input: VecDeque<String>) -> Result<Box<Self>> {
        anyhow::ensure!(input.len() == 1, "too many argument for get");
        Ok(Box::new(Get {
            key: input[0].to_owned(),
        }))
    }
}

impl Execution for Get {
    fn exec(&self, data: &mut DataStorage) -> Value {
        match data.get(&self.key) {
            Some(data_ttl) => {
                if let Some(v) = data_ttl.get() {
                    Value::Bulk(v)
                } else {
                    // data expired
                    data.remove(&self.key);
                    Value::Null
                }
            }
            None => Value::Null,
        }
    }
}
