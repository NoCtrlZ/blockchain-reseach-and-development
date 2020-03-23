use std::net::TcpListener;
use std::net::TcpStream;
use serde_json::json;
use crate::lamport::Wallet;
use crate::request::Request;
use crate::router::{Router, Handler};
use crate::blockchain::{Blockchain, Block, Transaction};
use crate::p2p::{Network, Add, NetworkInfo};
use crate::response::{Response, PREFIX};
use crate::utxo::Utxo;

pub struct Server {
    router: Router,
    blockchain: Blockchain,
    network: Network,
    wallet: Wallet,
    utxo: Utxo
}

impl Server {
    pub fn new(router: Router) -> Server {
        let default_difficulty = 3;
        let wallet = Wallet::new();
        let network = Network::new();
        let utxo = Utxo::new();
        println!("the address is {:?}", &wallet.get_address());
        let mut server = Server {
            router: router,
            blockchain: Blockchain {
                entity: Vec::new(),
                transactions: Vec::new(),
                difficulty: default_difficulty,
            },
            network: network,
            wallet: wallet,
            utxo: utxo
        };
        server.blockchain.create_genesis_block();
        server
    }

    pub fn start(&mut self) {
        let addr = self.network.endpoint.clone();
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

    pub fn get_all(&mut self, req: Request) -> Response {
        let address = self.wallet.get_address();
        Response {
            prefix: PREFIX.to_string(),
            body: json!({
                "blockchain": self.blockchain,
                "network": self.network,
                "address": address
            }).to_string()
        }
    }

    pub fn get_blockchain(&mut self, req: Request) -> Response {
        let whole_blockchain = self.blockchain.blockchain_json();
        Response {
            prefix: PREFIX.to_string(),
            body: whole_blockchain.to_string()
        }
    }

    pub fn get_network(&mut self, req: Request) -> Response {
        let whole_network = self.network.network_json();
        Response {
            prefix: PREFIX.to_string(),
            body: whole_network.to_string()
        }
    }

    pub fn create_new_block(&mut self, req: Request) -> Response {
        // println!("create new block");
        let block = self.blockchain.proof_of_work();
        self.utxo.admin_transfer(&self.wallet.get_address());
        self.network.block_broadcast(block.clone());
        Response {
            prefix: PREFIX.to_string(),
            body: json!(block).to_string()
        }
    }

    pub fn add_block(&mut self, req: Request) -> Response {
        let block: Block = serde_json::from_str(&req.body).unwrap();
        self.blockchain.entity.push(block.clone());
        self.blockchain.transactions.clear();
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

    pub fn add(&mut self, req: Request) -> Response {
        let body: Add = serde_json::from_str(&req.body).unwrap();
        println!("add {} to network", body.endpoint);
        self.network.nodes.push(body.endpoint);
        Response {
            prefix: PREFIX.to_string(),
            body: "Ok".to_string()
        }
    }

    pub fn join(&mut self, req: Request) -> Response {
        let body: Add = serde_json::from_str(&req.body).unwrap();
        println!("add {} to network", body.endpoint);
        let current_nodes = self.network.nodes.clone();
        self.network.broadcast(&body.endpoint);
        self.network.nodes.push(body.endpoint.clone());
        Response {
            prefix: PREFIX.to_string(),
            body: json!(current_nodes).to_string()
        }
    }

    pub fn get_nodes(&mut self, req: Request) -> Response {
        let info = NetworkInfo {
            endpoint: self.network.endpoint.clone(),
            nodes: self.network.nodes.clone()
        };
        let network = json!(info);
        Response {
            prefix: PREFIX.to_string(),
            body: network.to_string()
        }
    }

    pub fn balance(&mut self, req: Request) -> Response {
        let address = self.wallet.get_address();
        let balance = self.utxo.balance(&address);
        Response {
            prefix: PREFIX.to_string(),
            body: balance.to_string()
        }
    }
}
