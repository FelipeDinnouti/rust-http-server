use std::collections::HashMap;

pub type Headers = HashMap<String, String>;

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: Option<String>,
    pub version: String,
    pub headers: Headers,
    pub body: Vec<u8>
}

pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut serialized: Vec<u8> = Vec::new();
        
        // First part of the response, status and status text, e.g. HTTP/1.1 200 OK 
        serialized.extend_from_slice(format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text).as_bytes());

        // The headers of the response (Content-Type, Content-Length)
        for (k, v) in &self.headers {
            serialized.extend_from_slice(format!("{}: {}\r\n", k, v).as_bytes());
        }   

        // Mandatory blank line
        serialized.extend_from_slice("\r\n".as_bytes()); // Or b"\r\n" (byte conversion prefix)

        // Body is already a vector of bytes
        serialized.extend_from_slice(&self.body);

        serialized // Implicit return
    }
}