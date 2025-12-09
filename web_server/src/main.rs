use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (status_line, filename) =
    // Rust can use if expressions to return values, so in this case we can populate
    // the tuple with the appropriate values based on the condition
        if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    // write_all needs a mutable reference to stream as parameter as it changes the stream state
    stream.write_all(response.as_bytes()).unwrap();


}
