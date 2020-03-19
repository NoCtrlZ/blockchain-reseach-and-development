use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rand::Rng;

use crate::request::Request;
use crate::response::{Response, prefix};
use crate::router::{Router, Handler};

pub struct Network {
    endpoint: String,
    nodes: Vec<String>,
    router: Router
}

impl Network {
    pub fn new(router: Router) {
        let mut endpoint = "127.0.0.1:".to_string();
        endpoint.push_str(&random_port());
        println!("{:?}", &endpoint);
        let network = Network {
            endpoint: endpoint,
            nodes: Vec::new(),
            router: router
        };
        let listener = TcpListener::bind(&network.endpoint).unwrap();
        for stream in listener.incoming() {
            let response = network.handle(&mut stream.unwrap());
        }
    }

    fn handle(&self, stream: &mut TcpStream) {
        let req = Request::parse(stream);
        // println!("{:?}", self.router.routes[0].path);
        for route in &self.router.routes {
            if route.method == req.method && route.path == req.path {
                self.response(stream, route.handler, req);
                break;
            }
        }
    }

    fn response(&self, stream: &mut TcpStream, handler: Handler, req: Request) {
        let response = (handler)(self, req);
        response.write(stream);
    }

    pub fn compiler(&self, req: Request) -> Response {
        Response {
            prefix: prefix::PREFIX.to_string(),
            body: "Test".to_string(),
        }
    }
}

fn random_port() -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(1024, 9000).to_string()
}
