use std::net::TcpListener;
use std::net::TcpStream;

use crate::request;
use crate::router::{Router, Handler};
use crate::blockchain::Blockchain;
use crate::p2p::{Network, Node};

pub struct Server {
    router: Router,
    blockchain: Blockchain,
    network: Network
}

impl Server {
    pub fn new(router: Router) -> Server {
        let default_difficulty = 3;
        let default_port = 3000;
        let mut server = Server {
            router: router,
            blockchain: Blockchain {
                entity: Vec::new(),
                transactions: Vec::new(),
                difficulty: default_difficulty,
            },
            network: Network {
                nodes: Vec::new(),
                host: [127, 0, 0, 1],
            }
        };
        let original_node = Node {
            port: default_port
        };
        server.network.nodes.push(original_node);
        server
    }

    pub fn start(&self) {
        let addr = self.network.get_address();
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            self.handle(&mut stream.unwrap());
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

    fn response(&self, stream: &mut TcpStream, handler: Handler, req: request::Request) {
        let response = handler(self.blockchain.clone(), req);
        response.write(stream);
    }
}
