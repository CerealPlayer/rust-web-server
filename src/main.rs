use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (response, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 Ok", "hello.html")
    } else if request_line == "GET /styles.css HTTP/1.1" {
        ("HTTP/1.1 200 Ok", "styles.css")
    } else {
        ("HTTP/1.1 400 Not found", "404.html")
    };
    let body = fs::read_to_string(filename).unwrap();
    let length = body.len();
    let response = format!("{response}\r\nContent-length: {length}\r\n\r\n{body}");
    stream.write_all(response.as_bytes()).unwrap();
}
