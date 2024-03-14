use std::collections::VecDeque;
use std::time;

use crate::data_watcher::execution::Execution;
use crate::data_watcher::{DataStorage, DataTTL};

use anyhow::Result;
use resp::Value;

//https://redis.io/commands/set/
#[derive(Default, PartialEq, Debug)]
pub struct Set {
    key: String,
    value: String,
}

impl Set {
    pub fn parse(input: VecDeque<String>) -> Result<Box<Self>, Value> {
        if input.len() != 2 {
            return Result::Err(Value::Error("command parse error".to_string()));
        }
        Ok(Box::new(Set {
            key: input[0].to_owned(),
            value: input[1].to_owned(),
        }))
    }
}

impl Execution for Set {
    fn exec(&self, data: &mut DataStorage) -> Value {
        data.insert(
            self.key.to_owned(),
            DataTTL::new(self.value.to_owned(), time::Duration::default()),
        );
        Value::String("ok".to_string())
    }
}
