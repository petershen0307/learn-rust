use crate::data_watcher::execution::Execution;

use anyhow::Result;
use resp::Value;

#[derive(Default)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn parse(input: Vec<String>) -> Result<Self, Value> {
        if input.len() != 1 {
            return Result::Err(Value::Error("command parse error".to_string()));
        }
        Ok(Get {
            key: input[0].to_owned(),
        })
    }
}

impl Execution for Get {
    fn exec(&self, data: &mut crate::data_watcher::DataStorage) -> Value {
        match data.get(&self.key) {
            Some(v) => Value::String(v.to_string()),
            None => Value::Null,
        }
    }
}
