use std::collections::HashMap;

pub type Headers = HashMap<String, String>;

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

impl Method {
    pub fn from_string(s: &str) -> Method {
        match s {
            "GET" => return Method::GET,
            "POST" => return Method::POST,
            "PUT" => return Method::PUT,
            "PATCH" => return Method::PATCH,
            "DELETE" => return Method::DELETE,
            _ => panic!("INVALID REQUEST HEADER") // TODO: error handling, never use a panic
        }
    }
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

impl Request {
    pub fn new(data: &Vec<u8>) -> Request {
        let data_string = String::from_utf8_lossy(&data);
        let request_parts: Vec<&str> = data_string.split("\r\n").collect();

        // The first line of the request, with the method, path, query and version
        let request_line: Vec<&str> = request_parts[0].split_whitespace().collect();

        // The headers of the request, a key value pair.
        let mut headers: HashMap<String, String> = HashMap::new();

        for line in &request_parts[1..] {
            // End of headers
            if line.to_string() == "" {
                break;
            }
            // Add header as key value pair in the headers
            if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.to_string(), value.to_string());
            } 
        }

        let mut received_body: Vec<u8> = Vec::new();
        
        // Only set the received body if it received a body (body is optional)
        if let Some(body) = &request_parts.last() {
            println!("body: {}", body);
            received_body = body.as_bytes().to_vec();
        }

        Request { method: Method::from_string(request_line[0]), path: request_line[1].to_string(), query: Some("QUERY TODO".to_owned()),  version: request_line[2].to_string(), headers: headers, body: received_body }
    }
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