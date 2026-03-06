use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
 
use crossbeam_channel::{select, unbounded, Receiver};

mod cli_listener;
mod threadpool;
mod http_structure;
mod routing;

use cli_listener::start_cli_thread;
use threadpool::ThreadPool;
use http_structure::Request;
use routing::{Handler, Router};

pub struct Server {
    router: Router,
}

impl Server {

    fn new() -> Server {
        Server { router: Router::new() } 
    }

    fn serve(&self) {
        // Start listening for TCP connections
        let listener = TcpListener::bind("127.0.0.1:7878")
            .expect("Failed to bind address");

        // One way channels between threads - tx is sender, rx is receiver
        let (stream_tx, stream_rx) = unbounded();
        let (shutdown_tx, shutdown_rx) = unbounded();   

        start_cli_thread(shutdown_tx);
        println!("CLI Listener running...");

        // Starting the server
        println!("Server running at http://127.0.0.1:7878");

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        // Tries to send the stream
                        if stream_tx.send(stream).is_err() {    
                            break; // If it failed, sender was dropped.
                        }
                    }
                    Err(e) => {
                        eprintln!("Accept error: {}", e);
                        break;
                    }
                }
            }
        });

        let pool = ThreadPool::new(4);  

        loop {
            select! {
                recv(stream_rx) -> stream => {
                    let stream = stream.unwrap();
                    pool.execute(move || {
                        handle_connection(stream);
                    })
                }
                recv(shutdown_rx) -> _ => { 
                    println!("Shutdown message received, shutting down...");
                    break;
                }
            }
        }
    }
}


fn main() {
    let mut server = Server::new();

    server.serve();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Read the HTTP request bytes and store it in the buffer
    let size: usize = stream.read(&mut buffer).unwrap();

    // Parse the request
    let request: Request = Request::new(&buffer[0..size].to_vec());

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
