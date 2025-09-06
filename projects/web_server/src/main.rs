use std::fs::{self};
use std::io::{BufReader, prelude::*};
use std::net::TcpListener;

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for streams in listener.incoming() {
        let stream = streams.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
        println!("Shutting down");
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let response_content = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if response_content == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
