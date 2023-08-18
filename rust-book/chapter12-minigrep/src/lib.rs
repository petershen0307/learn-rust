use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // use reference slice not &Vec<String> is that reference slice can accept a dataset with length, not just vector, it can also accept an array
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            // 12.3 The Trade-Offs of Using clone
            // https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#the-trade-offs-of-using-clone
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

// Chapter 17 will talk about Box<dyn std::error::Error>>
// dyn==dynamic
// ? is told at Chapter 9, let caller to handle the the error
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    println!("Result:");
    let results = search(&config.query, &contents);
    for r in results {
        println!("{r}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
