use std::io::BufReader;

use crate::data_watcher::execution::Execution;
use crate::data_watcher::DataStorage;

use resp::Value;

#[derive(Default)]
pub struct Command {}

impl Command {
    pub const DOCS_SET: &'static str = "*2\r\n$3\r\nset\r\n*12\r\n$7\r\nsummary\r\n$90\r\nSets the string value of a key, ignoring its type. The key is created if it doesn't exist.\r\n$5\r\nsince\r\n$5\r\n1.0.0\r\n$5\r\ngroup\r\n$6\r\nstring\r\n$10\r\ncomplexity\r\n$4\r\nO(1)\r\n$7\r\nhistory\r\n*4\r\n*2\r\n$6\r\n2.6.12\r\n$44\r\nAdded the `EX`, `PX`, `NX` and `XX` options.\r\n*2\r\n$5\r\n6.0.0\r\n$27\r\nAdded the `KEEPTTL` option.\r\n*2\r\n$5\r\n6.2.0\r\n$42\r\nAdded the `GET`, `EXAT` and `PXAT` option.\r\n*2\r\n$5\r\n7.0.0\r\n$55\r\nAllowed the `NX` and `GET` options to be used together.\r\n$9\r\narguments\r\n*5\r\n*8\r\n$4\r\nname\r\n$3\r\nkey\r\n$4\r\ntype\r\n$3\r\nkey\r\n$12\r\ndisplay_text\r\n$3\r\nkey\r\n$14\r\nkey_spec_index\r\n:0\r\n*6\r\n$4\r\nname\r\n$5\r\nvalue\r\n$4\r\ntype\r\n$6\r\nstring\r\n$12\r\ndisplay_text\r\n$5\r\nvalue\r\n*10\r\n$4\r\nname\r\n$9\r\ncondition\r\n$4\r\ntype\r\n$5\r\noneof\r\n$5\r\nsince\r\n$6\r\n2.6.12\r\n$5\r\nflags\r\n*1\r\n+optional\r\n$9\r\narguments\r\n*2\r\n*8\r\n$4\r\nname\r\n$2\r\nnx\r\n$4\r\ntype\r\n$10\r\npure-token\r\n$12\r\ndisplay_text\r\n$2\r\nnx\r\n$5\r\ntoken\r\n$2\r\nNX\r\n*8\r\n$4\r\nname\r\n$2\r\nxx\r\n$4\r\ntype\r\n$10\r\npure-token\r\n$12\r\ndisplay_text\r\n$2\r\nxx\r\n$5\r\ntoken\r\n$2\r\nXX\r\n*12\r\n$4\r\nname\r\n$3\r\nget\r\n$4\r\ntype\r\n$10\r\npure-token\r\n$12\r\ndisplay_text\r\n$3\r\nget\r\n$5\r\ntoken\r\n$3\r\nGET\r\n$5\r\nsince\r\n$5\r\n6.2.0\r\n$5\r\nflags\r\n*1\r\n+optional\r\n*8\r\n$4\r\nname\r\n$10\r\nexpiration\r\n$4\r\ntype\r\n$5\r\noneof\r\n$5\r\nflags\r\n*1\r\n+optional\r\n$9\r\narguments\r\n*5\r\n*10\r\n$4\r\nname\r\n$7\r\nseconds\r\n$4\r\ntype\r\n$7\r\ninteger\r\n$12\r\ndisplay_text\r\n$7\r\nseconds\r\n$5\r\ntoken\r\n$2\r\nEX\r\n$5\r\nsince\r\n$6\r\n2.6.12\r\n*10\r\n$4\r\nname\r\n$12\r\nmilliseconds\r\n$4\r\ntype\r\n$7\r\ninteger\r\n$12\r\ndisplay_text\r\n$12\r\nmilliseconds\r\n$5\r\ntoken\r\n$2\r\nPX\r\n$5\r\nsince\r\n$6\r\n2.6.12\r\n*10\r\n$4\r\nname\r\n$17\r\nunix-time-seconds\r\n$4\r\ntype\r\n$9\r\nunix-time\r\n$12\r\ndisplay_text\r\n$17\r\nunix-time-seconds\r\n$5\r\ntoken\r\n$4\r\nEXAT\r\n$5\r\nsince\r\n$5\r\n6.2.0\r\n*10\r\n$4\r\nname\r\n$22\r\nunix-time-milliseconds\r\n$4\r\ntype\r\n$9\r\nunix-time\r\n$12\r\ndisplay_text\r\n$22\r\nunix-time-milliseconds\r\n$5\r\ntoken\r\n$4\r\nPXAT\r\n$5\r\nsince\r\n$5\r\n6.2.0\r\n*10\r\n$4\r\nname\r\n$7\r\nkeepttl\r\n$4\r\ntype\r\n$10\r\npure-token\r\n$12\r\ndisplay_text\r\n$7\r\nkeepttl\r\n$5\r\ntoken\r\n$7\r\nKEEPTTL\r\n$5\r\nsince\r\n$5\r\n6.0.0\r\n";
    pub const DOCS_GET: &'static str = "*2\r\n$3\r\nget\r\n*10\r\n$7\r\nsummary\r\n$34\r\nReturns the string value of a key.\r\n$5\r\nsince\r\n$5\r\n1.0.0\r\n$5\r\ngroup\r\n$6\r\nstring\r\n$10\r\ncomplexity\r\n$4\r\nO(1)\r\n$9\r\narguments\r\n*1\r\n*8\r\n$4\r\nname\r\n$3\r\nkey\r\n$4\r\ntype\r\n$3\r\nkey\r\n$12\r\ndisplay_text\r\n$3\r\nkey\r\n$14\r\nkey_spec_index\r\n:0\r\n";
    pub const DOCS_DEL: &'static str = "*2\r\n$3\r\ndel\r\n*10\r\n$7\r\nsummary\r\n$25\r\nDeletes one or more keys.\r\n$5\r\nsince\r\n$5\r\n1.0.0\r\n$5\r\ngroup\r\n$7\r\ngeneric\r\n$10\r\ncomplexity\r\n$288\r\nO(N) where N is the number of keys that will be removed. When a key to remove holds a value other than a string, the individual complexity for this key is O(M) where M is the number of elements in the list, set, sorted set or hash. Removing a single key that holds a string value is O(1).\r\n$9\r\narguments\r\n*1\r\n*10\r\n$4\r\nname\r\n$3\r\nkey\r\n$4\r\ntype\r\n$3\r\nkey\r\n$12\r\ndisplay_text\r\n$3\r\nkey\r\n$14\r\nkey_spec_index\r\n:0\r\n$5\r\nflags\r\n*1\r\n+multiple\r\n";
    pub const DOCS: [&'static str; 3] = [Command::DOCS_SET, Command::DOCS_GET, Command::DOCS_DEL];
}

impl Execution for Command {
    fn exec(&self, _: &mut DataStorage) -> resp::Value {
        let r = Command::DOCS.map(|x| {
            resp::Decoder::new(BufReader::new(x.as_bytes()))
                .decode()
                .unwrap()
        });
        let mut output = Vec::new();
        for i in r {
            if let Value::Array(mut v) = i {
                output.append(&mut v);
            }
        }
        Value::Array(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec() {
        // arrange
        let cmd = Command::default();
        // act
        let result = cmd.exec(&mut DataStorage::new());
        // assert
        let mut doc_strings = Vec::new();
        if let Value::Array(mut r) = result {
            while !r.is_empty() {
                let mut v = Vec::new();
                // pop last one and insert to first
                v.insert(0, r.pop().unwrap());
                v.insert(0, r.pop().unwrap());
                // to fix the order to be set, get, del
                doc_strings.insert(0, String::from_utf8(Value::Array(v).encode()).unwrap());
            }
            assert_eq!(Command::DOCS.to_vec(), doc_strings);
        } else {
            unreachable!();
        }
    }
}
