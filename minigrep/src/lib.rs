use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

// We tell Rust that the data we return by search function, will live as long as
// the data passed into the search function in the contents argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // we use &'static str because the error message we return has a static lifetime
    // is the common approach for simple error handling in Rust
    // means the string doesn't come from a variable or computed at runtime
    // its hardcoded into the program, specifically in this function
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// Box<dyn Error> is a trait object that can represent any error type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? operator returns to the caller the error so the caller can decide how to handle it
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}