// use $env:RUST_BACKTRACE=1 cargo run to enable backtrace
// when using Windows powershell

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This will cause the program to panic and terminate pointing to this
    // line number in the stack trace.
    //panic!("crash and burn");

    let v = vec![1, 2, 3];
    //v[99];



    // Recoverable Errors with Result<T, E>
    // Contains either Ok value or an Err value.
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        // File::open if Ok returns the file handle so we need to
        // bind it to a variable. and then => that's what gets returned
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match  File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!(
                        "Problem creating the file: {:?}",
                        e
                    ),
                }
            }
            other_error => {
                panic!(
                    "Problem opening the file: {:?}",
                    other_error
                );
            }
        },
    };


    // THIS IS A SHORTCUT FOR THE ABOVE MATCH STATEMENT
    let greeting_file2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
        File::create("hello2.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
    });

    // USING UNWRAP HELPER METHOD
    // Unwrap will return the value inside the Ok and if the result is an Err variant,
    // unwrap will call the panic! macro, so it's a helper method
    let greeting_file3 =  File::open("hello3.txt").unwrap();

    // USING EXPECT HELPER METHOD
    // Expect is similar to unwrap but lets you specify the panic message
    let greeting_file4 =  File::open("hello4.txt")
        .expect("Failed to open hello4.txt");



    // We can do this on Main because we changed the return type of main to Result
    // but the error type of main has to be Box<dyn std::error::Error> meaning
    // it can return any type of error.
    let greeting_file = File::open("hello.txt")?;
    Ok(())

    // Generally both unwrap and expect methods are used for prototyping unless we can ensure
    // that we will never have an Err variant, then we can use unwrap and document the
    // reason.
}


// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// A shortcut for propagating errors with ? operator
fn read_username_from_file_shourtcut() ->  Result<String, io::Error>  {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    // Puts the result of read_to_string into username
    // if it's an error it will return the error from the function
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


// Even shorter version
fn read_username_from_file_shortest() ->  Result<String, io::Error>  {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// So basically the ? operator is a shortcut for propagating errors
// It can only be used in functions that return Result<T, E> or Option<T>
// because it needs to return the error or None value to the caller
// Takes a Result<T, E> value
//
// If it's Ok(value): unwraps and gives you the value
//
// If it's Err(error): immediately returns Err(error) from the current function

// To use the ? operator with functions that return Option<T> or Result<T, E>

// Reading a file into a string with fs::read_to_string function
fn read_username_from_file_fs() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
