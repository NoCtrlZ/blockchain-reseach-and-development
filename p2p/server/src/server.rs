use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::request;
use crate::response;
use crate::router;

pub struct Server {
    router: router::Router,
}

impl Server {
    pub fn new(router: router::Router) -> Server {
        Server { router: router }
    }

    pub fn start(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let response = self.handle(&mut stream.unwrap());
        }
    }

    fn handle(&self, stream: &mut TcpStream) {
        let req = request::Request::parse(stream);
        // println!("{:?}", self.router.routes[0].path);
        for route in &self.router.routes {
            if route.method == req.method && route.path == req.path {
                self.response(stream, route.handler, req);
                break;
            }
        }
    }

    fn response(&self, stream: &mut TcpStream, handler: router::Handler, req: request::Request) {
        let response = (handler)(req);
        response.write(stream);
    }
}
