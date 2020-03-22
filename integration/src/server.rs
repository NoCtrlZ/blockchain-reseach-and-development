use std::net::TcpListener;
use std::net::TcpStream;
use serde_json::json;
use crate::lamport::Wallet;
use crate::request::Request;
use crate::router::{Router, Handler};
use crate::blockchain::{Blockchain, Transaction};
use crate::p2p::{Network, Node};
use crate::response::{Response, PREFIX};

pub struct Server {
    router: Router,
    blockchain: Blockchain,
    network: Network,
    wallet: Wallet
}

impl Server {
    pub fn new(router: Router) -> Server {
        let default_difficulty = 3;
        let default_port = 3000;
        let wallet = Wallet::new();
        println!("this node address is {:?}", &wallet.get_address());
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
            },
            wallet: wallet
        };
        let original_node = Node {
            port: default_port
        };
        server.blockchain.create_genesis_block();
        server.network.nodes.push(original_node);
        server
    }

    pub fn start(&mut self) {
        let addr = self.network.get_address();
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            self.handle(&mut stream.unwrap());
        }
    }

    fn handle(&mut self, stream: &mut TcpStream) {
        let req = Request::parse(stream);
        // println!("{:?}", req);
        for route in &self.router.routes {
            if route.method == req.method && route.path == req.path {
                self.response(stream, route.handler, req);
                break;
            }
        }
    }

    fn response(&mut self, stream: &mut TcpStream, handler: Handler, req: Request) {
        // println!("response");
        let response = handler(self, req);
        response.write(stream);
    }

    pub fn get_blockchain(&mut self, req: Request) -> Response {
        let whole_blockchain = self.blockchain.blockchain_json();
        Response {
            prefix: PREFIX.to_string(),
            body: whole_blockchain.to_string()
        }
    }

    pub fn create_new_block(&mut self, req: Request) -> Response {
        // println!("create new block");
        let block = self.blockchain.proof_of_work();
        Response {
            prefix: PREFIX.to_string(),
            body: json!(block).to_string()
        }
    }

    pub fn send_transaction(&mut self, req: Request) -> Response {
        // println!("{:?}", req);
        let transaction: Transaction = serde_json::from_str(&req.body).unwrap();
        // println!("{:?}", transaction);
        let transaction_json = json!(&transaction);
        self.blockchain.transactions.push(transaction);
        Response {
            prefix: PREFIX.to_string(),
            body: transaction_json.to_string()
        }
    }
}
