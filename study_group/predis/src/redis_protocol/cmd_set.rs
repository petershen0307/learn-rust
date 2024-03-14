use std::collections::VecDeque;
use std::time;

use crate::data_watcher::execution::Execution;
use crate::data_watcher::{DataStorage, DataTTL};

use anyhow::Result;
use resp::Value;

// https://redis.io/commands/set/
#[derive(Default, PartialEq, Debug)]
pub struct Set {
    key: String,
    value: String,
    get: Option<()>,
    ttl_state: Option<TTLState>,
}

#[derive(PartialEq, Debug)]
enum TTLState {
    Ex(time::Duration),
    Px(time::Duration),
    Exat(time::Duration),
    Pxat(time::Duration),
    KeepTTL,
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
                "ex" => {
                    // EX seconds -- Set the specified expire time, in seconds (a positive integer).
                    if let Some(ttl) = input.pop_front() {
                        if let Ok(ttl_u64) = ttl.parse::<u64>() {
                            set_obj.ttl_state =
                                Some(TTLState::Ex(time::Duration::from_secs(ttl_u64)));
                        } else {
                            return Err(Value::Error("ex value is not integer".to_string()));
                        }
                    } else {
                        return Err(Value::Error("ex without value".to_string()));
                    }
                }
                "px" => {
                    // PX milliseconds -- Set the specified expire time, in milliseconds (a positive integer).
                    if let Some(ttl) = input.pop_front() {
                        if let Ok(ttl_u64) = ttl.parse::<u64>() {
                            set_obj.ttl_state =
                                Some(TTLState::Px(time::Duration::from_millis(ttl_u64)));
                        } else {
                            return Err(Value::Error("px value is not integer".to_string()));
                        }
                    } else {
                        return Err(Value::Error("px without value".to_string()));
                    }
                }
                "exat" => {
                    // EXAT timestamp-seconds -- Set the specified Unix time at which the key will expire, in seconds (a positive integer).
                    if let Some(ttl) = input.pop_front() {
                        if let Ok(ttl_u64) = ttl.parse::<u64>() {
                            set_obj.ttl_state =
                                Some(TTLState::Exat(time::Duration::from_secs(ttl_u64)));
                        } else {
                            return Err(Value::Error("exat value is not integer".to_string()));
                        }
                    } else {
                        return Err(Value::Error("exat without value".to_string()));
                    }
                }
                "pxat" => {
                    // PXAT timestamp-milliseconds -- Set the specified Unix time at which the key will expire, in milliseconds (a positive integer).
                    if let Some(ttl) = input.pop_front() {
                        if let Ok(ttl_u64) = ttl.parse::<u64>() {
                            set_obj.ttl_state =
                                Some(TTLState::Pxat(time::Duration::from_millis(ttl_u64)));
                        } else {
                            return Err(Value::Error("pxat value is not integer".to_string()));
                        }
                    } else {
                        return Err(Value::Error("pxat without value".to_string()));
                    }
                }
                "keepttl" => {
                    // KEEPTTL -- Retain the time to live associated with the key.
                    set_obj.ttl_state = Some(TTLState::KeepTTL)
                }
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
        let mut data_ttl = DataTTL::new(self.value.to_owned());
        if let Some(ttl_state) = &self.ttl_state {
            data_ttl = match ttl_state {
                TTLState::Ex(s) => data_ttl.ttl(s),
                TTLState::Px(m) => data_ttl.ttl(m),
                TTLState::Exat(s) => data_ttl.expired(s),
                TTLState::Pxat(m) => data_ttl.expired(m),
                TTLState::KeepTTL => {
                    if let Some(v) = data.get(&self.key) {
                        v.to_owned().update(self.value.to_owned())
                    } else {
                        data_ttl
                    }
                }
            }
        }
        data.insert(self.key.to_owned(), data_ttl);
        return_value
    }
}

#[cfg(test)]
mod test_parse {
    use core::time;
    use std::collections::VecDeque;

    use crate::redis_protocol::cmd_set::TTLState;

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
    #[test]
    fn test_parse_get_ex_success() {
        // arrange
        let input = VecDeque::from(vec!["k", "v", "ex", "5", "get"]);
        let input = input.iter().map(|x| x.to_string()).collect();
        let expected = Set {
            key: "k".to_string(),
            value: "v".to_string(),
            get: Some(()),
            ttl_state: Some(TTLState::Ex(time::Duration::from_secs(5))),
        };
        // act
        let result = Set::parse(input);
        // assert
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(Box::new(expected), result);
    }
    #[test]
    fn test_parse_get_px_success() {
        // arrange
        let input = VecDeque::from(vec!["k", "v", "get", "px", "5"]);
        let input = input.iter().map(|x| x.to_string()).collect();
        let expected = Set {
            key: "k".to_string(),
            value: "v".to_string(),
            get: Some(()),
            ttl_state: Some(TTLState::Px(time::Duration::from_millis(5))),
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
    use super::Set;
    use crate::{
        data_watcher::{execution::Execution, DataStorage},
        redis_protocol::cmd_set::TTLState,
    };
    use resp::Value;
    use std::{
        thread,
        time::{self, UNIX_EPOCH},
    };

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
    #[test]
    fn test_exec_ttl_success() {
        // arrange
        let ttl = time::Duration::from_secs(1);
        let set_obj = Set {
            key: "k".to_string(),
            value: "v".to_string(),
            ttl_state: Some(TTLState::Ex(ttl.to_owned())),
            ..Default::default()
        };
        let mut data = DataStorage::new();

        // act
        let result = set_obj.exec(&mut data);
        // assert
        assert_eq!(Value::String("ok".to_string()), result);
        let output = data.get(&set_obj.key).unwrap();
        assert_eq!(set_obj.value, output.get().unwrap());
        thread::sleep(ttl);
        assert!(output.get().is_none());
    }
    #[test]
    fn test_exec_keep_ttl_success() {
        // arrange
        let ttl = time::Duration::from_secs(1);
        let set_obj = Set {
            key: "k".to_string(),
            value: "v".to_string(),
            ttl_state: Some(TTLState::Exat(
                time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + ttl,
            )),
            ..Default::default()
        };
        let set_obj2 = Set {
            key: "k".to_string(),
            value: "v2".to_string(),
            ttl_state: Some(TTLState::KeepTTL),
            ..Default::default()
        };
        let mut data = DataStorage::new();

        // act
        let result = set_obj.exec(&mut data);
        assert_eq!(Value::String("ok".to_string()), result);
        let result = set_obj2.exec(&mut data);
        assert_eq!(Value::String("ok".to_string()), result);
        // assert
        let output = data.get(&set_obj2.key).unwrap();
        assert_eq!(set_obj2.value, output.get().unwrap());
        thread::sleep(ttl);
        assert!(output.get().is_none());
    }
    #[test]
    fn test_exec_ttl_pxat_success() {
        // arrange
        let ttl = time::Duration::from_secs(1);
        let set_obj = Set {
            key: "k".to_string(),
            value: "v".to_string(),
            ttl_state: Some(TTLState::Pxat(
                time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + ttl,
            )),
            ..Default::default()
        };
        let mut data = DataStorage::new();

        // act
        let result = set_obj.exec(&mut data);
        // assert
        assert_eq!(Value::String("ok".to_string()), result);
        let output = data.get(&set_obj.key).unwrap();
        assert_eq!(set_obj.value, output.get().unwrap());
        thread::sleep(ttl);
        assert!(output.get().is_none());
    }
}
