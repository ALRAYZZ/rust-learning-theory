use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect(); // we annotate the type because collect() is ambiguous

    // Unwrap_or_else is a method that either unwraps the Result if it's Ok
    // or calls the provided closure if it's Err
    let config = Config::build(&args).unwrap_or_else((|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    }));


    // Different pattern to handle errors because the run function does not return
    // a value that we want to unwrap, since it returns Ok(()) on success
    // so we just match on the Result directly
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// Box<dyn Error> is a trait object that can represent any error type
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? operator returns to the caller the error so the caller can decide how to handle it
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    // we use &'static str because the error message we return has a static lifetime
    // is the common approach for simple error handling in Rust
    // means the string doesn't come from a variable or computed at runtime
    // its hardcoded into the program, specifically in this function
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}