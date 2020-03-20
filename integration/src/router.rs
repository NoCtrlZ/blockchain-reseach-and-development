use crate::request;
use crate::response;
use crate::blockchain;
use crate::server::Server;

pub mod method {
    pub const GET: &str = "GET";
    pub const POST: &str = "POST";
}

pub type Handler = fn(&mut Server, req: request::Request) -> response::Response;

pub struct Route {
    pub path: String,
    pub method: String,
    pub handler: Handler,
}

pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn register(&mut self, method: &str, path: &str, handler: Handler) {
        let route = Route {
            method: String::from(method),
            path: String::from(path),
            handler: handler,
        };
        self.routes.push(route)
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        const METHOD: &str = method::GET;
        self.register(METHOD, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        const METHOD: &str = method::POST;
        self.register(METHOD, path, handler);
    }
}
