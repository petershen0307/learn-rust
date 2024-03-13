use crate::data_watcher::execution::Execution;

use anyhow::Result;
use resp::Value;

#[derive(Default)]
pub struct Set {
    key: String,
    value: String,
}

impl Set {
    pub fn parse(input: Vec<String>) -> Result<Self, Value> {
        if input.len() != 2 {
            return Result::Err(Value::Error("command parse error".to_string()));
        }
        Ok(Set {
            key: input[0].to_owned(),
            value: input[1].to_owned(),
        })
    }
}

impl Execution for Set {
    fn exec(&self, data: &mut crate::data_watcher::DataStorage) -> Value {
        data.insert(self.key.to_owned(), self.value.to_owned());
        Value::String("ok".to_string())
    }
}
