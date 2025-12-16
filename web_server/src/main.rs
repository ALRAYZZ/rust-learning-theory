use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // Creates 4 worker threads ready to handle conns

    // Infinite loop accepting incoming TCP connections
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // Instead of spawning a new thread per connection, reuse existing workers
        pool.execute(|| {
            handle_connection(stream);
        });

        println!("Connection closed.");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // Read only the first line (HTTP request line: "GET / HTTP/1.1")
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    // Pattern matching on request to determine response
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            // Simulates slow request - threadpool prevents this from blocking other connections
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Construct HTTP response with headers and body
    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    // write_all needs a mutable reference to stream as parameter as it changes the stream state
    stream.write_all(response.as_bytes()).unwrap();


}
