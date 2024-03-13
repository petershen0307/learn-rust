use std::collections::VecDeque;

use crate::data_watcher::execution::Execution;
use crate::data_watcher::DataStorage;

use anyhow::Result;
use resp::Value;

#[derive(Default, PartialEq, Debug)]
pub struct Del {
    key: String,
}

impl Del {
    pub fn parse(input: VecDeque<String>) -> Result<Box<Self>, Value> {
        if input.len() != 1 {
            return Result::Err(Value::Error("command parse error".to_string()));
        }
        Ok(Box::new(Del {
            key: input[0].to_owned(),
        }))
    }
}

impl Execution for Del {
    fn exec(&self, data: &mut DataStorage) -> Value {
        match data.remove(&self.key) {
            Some(_) => Value::Integer(1),
            None => Value::Integer(0),
        }
    }
}
