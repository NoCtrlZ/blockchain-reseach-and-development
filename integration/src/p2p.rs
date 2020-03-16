use std::net::SocketAddr;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;

#[derive(Debug)]
pub struct Network {
    pub nodes: Vec<Node>,
    pub host: [i32; 4]
}

#[derive(Debug)]
pub struct Node {
    port: u16
}

pub async fn peer_to_peer(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new("hello world man".into())),
        (&Method::POST, "/check_all") => {
            println!("{:?}", req);
            Ok(Response::new("hello world man".into()))
        },
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

impl Network {
    pub fn new() -> Network {
        let mut network = Network {
            nodes: Vec::new(),
            host: [127, 0, 0, 1]
        };
        let original_node = Node {
            port: 3000
        };
        network.nodes.push(original_node);
        network
    }

    pub fn get_address(self) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], self.nodes[0].port))
    }
}
