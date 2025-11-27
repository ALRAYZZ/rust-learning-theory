use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // env::args() returns an iterator of the command line arguments so we can use
    // it directly in the build function
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    // Different pattern to handle errors because the run function does not return
    // a value that we want to unwrap, since it returns Ok(()) on success
    // so we just match on the Result directly
    // If run config returns an Err value, we bind it to e and execute the block
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
