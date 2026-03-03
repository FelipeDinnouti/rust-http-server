## Roadmap

1. Basic HTTP Serve
2. Threadpool: serve multiple clients -> Upgrade to Async later.
3. HTTP Parsing
4. Routing logic
5. Middlewares
6. Database connection


## Architecture

Main thread: Listens for incoming TCP connections.
For each receiving connection we send the `TcpStream` to a worker thread.

1. The worker thread will parse the HTTP request into a `Request` object 
2. Find the appropriate handler function from the main Router based on the request path and method, 
3. And call that handler function with the request data, obtaining a response. 
4. And the response is then written to the TcpStream and the connection is closed.

### Handler Trait

This trait is used to enable any function, closure or struct that meet the bound to be used as a Handler function in a Router.

The bounds are `Send + Sync + 'static`, this ensures they can be safely used across threads.

It is implemented as a blanket implementation for every F that takes a Request and returns a Response: `F: Fn(Request) -> Response + Send + Sync + 'static,`
