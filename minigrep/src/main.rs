use std::env;
use std::process;

use minigrep::Config;

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
    // If run config returns an Err value, we bind it to e and execute the block
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
