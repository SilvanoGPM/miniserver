use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread, time,
};

use miniserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, content_type, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("200 OK", "application/json", "example.json"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(time::Duration::from_secs(5));
            ("200 OK", "application/json", "example.json")
        }
        _ => ("404 NOT FOUND", "text/html", "404.html"),
    };

    let status_line = format!("HTTP/1.1 {status}");
    let content = fs::read_to_string(filename).unwrap();
    let content_length = content.len();

    let headers = format!("Content-Length: {content_length}, Content-Type: {content_type}")
        .replace(", ", "\r\n");

    let response = format!("{status_line}\r\n{headers}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
