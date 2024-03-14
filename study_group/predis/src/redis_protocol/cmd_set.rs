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
    get: Option<()>,
}

impl Set {
    pub fn parse(mut input: VecDeque<String>) -> Result<Box<Self>, Value> {
        if input.len() < 2 {
            return Result::Err(Value::Error("command parse error".to_string()));
        }
        let mut set_obj = Set {
            key: input.pop_front().unwrap(),
            value: input.pop_front().unwrap(),
            ..Default::default()
        };
        while let Some(token) = input.pop_front() {
            match token.to_lowercase().as_str() {
                "get" => set_obj.get = Some(()),
                _ => {}
            }
        }
        Ok(Box::new(set_obj))
    }
}

impl Execution for Set {
    fn exec(&self, data: &mut DataStorage) -> Value {
        let return_value = if self.get.is_some() {
            if let Some(v) = data.get(&self.key) {
                Value::String(v.get().unwrap())
            } else {
                Value::Null
            }
        } else {
            Value::String("ok".to_string())
        };
        data.insert(
            self.key.to_owned(),
            DataTTL::new(self.value.to_owned(), time::Duration::default()),
        );
        return_value
    }
}

#[cfg(test)]
mod test_parse {
    use std::collections::VecDeque;

    use super::Set;

    #[test]
    fn test_parse_success() {
        // arrange
        let input = VecDeque::from(vec!["k", "v"]);
        let input = input.iter().map(|x| x.to_string()).collect();
        let expected = Set {
            key: "k".to_string(),
            value: "v".to_string(),
            ..Default::default()
        };
        // act
        let result = Set::parse(input);
        // assert
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(Box::new(expected), result);
    }
    #[test]
    fn test_parse_get_success() {
        // arrange
        let input = VecDeque::from(vec!["k", "v", "get"]);
        let input = input.iter().map(|x| x.to_string()).collect();
        let expected = Set {
            key: "k".to_string(),
            value: "v".to_string(),
            get: Some(()),
            ..Default::default()
        };
        // act
        let result = Set::parse(input);
        // assert
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(Box::new(expected), result);
    }
}

#[cfg(test)]
mod test_exec {

    use resp::Value;

    use crate::data_watcher::{execution::Execution, DataStorage};

    use super::Set;

    #[test]
    fn test_exec_success() {
        // arrange
        let set_obj = Set {
            key: "k".to_string(),
            value: "v".to_string(),
            ..Default::default()
        };
        let mut data = DataStorage::new();

        // act
        let result = set_obj.exec(&mut data);
        // assert
        assert_eq!(Value::String("ok".to_string()), result);
        assert_eq!(
            set_obj.value,
            data.get(&set_obj.key).unwrap().get().unwrap()
        );
    }
    #[test]
    fn test_exec_get_success() {
        // arrange
        let set_obj = Set {
            key: "k".to_string(),
            value: "v".to_string(),
            get: Some(()),
            ..Default::default()
        };
        let set_obj2 = Set {
            key: "k".to_string(),
            value: "v2".to_string(),
            get: Some(()),
            ..Default::default()
        };
        let mut data = DataStorage::new();

        // act
        let result = set_obj.exec(&mut data);
        assert_eq!(Value::Null, result);
        assert_eq!(
            set_obj.value,
            data.get(&set_obj.key).unwrap().get().unwrap()
        );
        let result2 = set_obj2.exec(&mut data);
        // assert
        assert_eq!(Value::String("v".to_string()), result2);
        assert_eq!(
            set_obj2.value,
            data.get(&set_obj2.key).unwrap().get().unwrap()
        );
    }
}
