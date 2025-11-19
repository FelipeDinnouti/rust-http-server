use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    // A single tcp connection with one client
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Failed to bind address");

    println!("Server running at http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");
        handle_connection(&mut stream);
    }
}

fn handle_connection(mut stream: &std::net::TcpStream) {
    let mut buffer = [0; 1024];

    // Read the HTTP request bytes
    stream.read(&mut buffer).unwrap();

    let get_request = b"GET / HTTP/1.1";

    let response = if buffer.starts_with(get_request) {
        http_response("Hello, Rust HTTP server!")
    } else {
        not_found()
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn http_response(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        body.len(),
        body
    )
}

fn not_found() -> String {
    let body = "<h1>404 - Not Found</h1>";
    format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        body.len(),
        body
    )
}
