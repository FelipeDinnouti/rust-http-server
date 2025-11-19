use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
pub use rust_http_server::threadpool::ThreadPool;

fn main() {
    // A TCP connection with one client
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Failed to bind address");

    println!("Server running at http://127.0.0.1:7878");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Read the HTTP request bytes
    stream.read(&mut buffer).unwrap();

    let get_request = b"GET / HTTP/1.1";

    let response = if buffer.starts_with(get_request) {
        http_response("Hello world from a Rust HTTP Server!")
    } else {
        not_found()
    };

    stream.write(response.as_bytes()).unwrap(); // Writing data to the stream
    stream.flush().unwrap(); // Flushing sends the data in the stream
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
