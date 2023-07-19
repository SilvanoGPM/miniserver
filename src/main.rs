use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
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

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let is_index_page = request_line == "GET / HTTP/1.1";

    let (status, content_type, filename) = if is_index_page {
        ("200 OK", "application/json", "example.json")
    } else {
        ("404 NOT FOUND", "text/html", "404.html")
    };

    let status_line = format!("HTTP/1.1 {status}");
    let content = fs::read_to_string(filename).unwrap();
    let content_length = content.len();

    let headers = format!("Content-Length: {content_length}, Content-Type: {content_type}")
        .replace(", ", "\r\n");

    let response = format!("{status_line}\r\n{headers}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
