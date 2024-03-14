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
    pub fn parse(input: VecDeque<String>) -> Result<Box<Self>, Value> {
        if input.len() != 1 {
            return Result::Err(Value::Error("command parse error".to_string()));
        }
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
                    Value::String(v)
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
