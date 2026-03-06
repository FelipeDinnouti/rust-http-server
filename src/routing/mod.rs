use std::collections::HashMap;
use rust_http_server::http_structure::{Request, Response};

// The Handler trait used to define handler functions used in Routers
pub trait Handler: Send + Sync + 'static {
    fn handle(&self, req: Request) -> Response;
}

// Blanket implementation for Handler traits for every F that meets the bounds (also called supertraits).
impl<F> Handler for F
where
    F: Fn(Request) -> Response + Send + Sync + 'static,
{
    fn handle(&self, req: Request) -> Response {
        (self)(req)
    }
}

pub struct Router {
    routes: HashMap<String, Box<dyn Handler>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    // It accepts any path that can be converted into a String, and any handler that implements Handler. 
    pub fn register(&mut self, path: impl Into<String>, handler: impl Handler) {
        self.routes.insert(path.into(), Box::new(handler));
    }

    pub fn route(&self, path: &str) -> Option<&dyn Handler> {
        // Transforms the &Box<dyn Handler> to a &dyn Handler (double dereference and then reference again, same as .as_ref()), so it can be called using .handle()
        self.routes.get(path).map(|b| &**b)
    }
}