use std::net::TcpListener;
use std::net::TcpStream;
use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::lamport::Wallet;
use crate::request::Request;
use crate::router::{Router, Handler};
use crate::blockchain::{Blockchain, Block};
use crate::p2p::{Network, Add, NetworkInfo, CurrentState};
use crate::response::{Response, PREFIX};
use crate::utxo::{Utxo, Transaction};

pub struct Server {
    router: Router,
    blockchain: Blockchain,
    network: Network,
    wallet: Wallet,
    utxo: Utxo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeInfo {
    pub block_length: usize,
    pub endpoint: String
}

impl Server {
    pub fn new(router: Router) -> Server {
        let default_difficulty = 3;
        let wallet = Wallet::new();
        let (network, blocks, transactions) = Network::new();
        let utxo = Utxo::new();
        println!("the address is {:?}", &wallet.get_address());
        let mut server = Server {
            router: router,
            blockchain: Blockchain {
                entity: blocks.clone(),
                transactions: transactions,
                difficulty: default_difficulty,
            },
            network: network,
            wallet: wallet,
            utxo: utxo
        };
        if blocks.len() == 0 {
            server.blockchain.create_genesis_block();
        }
        server
    }

    pub fn start(&mut self) {
        let addr = self.network.endpoint.clone();
        let listener = TcpListener::bind(addr).expect("fail to bind tcp listener");
        for stream in listener.incoming() {
            self.handle(&mut stream.expect("fail to read stream"));
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

    pub fn get_node_info(&mut self, req: Request) -> Response {
        let node_info = NodeInfo {
            block_length: self.blockchain.entity.len(),
            endpoint: self.network.endpoint.clone()
        };
        Response {
            prefix: PREFIX.to_string(),
            body: json!(node_info).to_string()
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
        self.network.block_broadcast(block.clone());
        let transaction = self.utxo.admin_transfer(&self.wallet.get_address());
        self.blockchain.transactions.push(transaction.clone());
        self.network.transaction_broadcast(transaction);
        Response {
            prefix: PREFIX.to_string(),
            body: json!(block).to_string()
        }
    }

    pub fn consensus(&mut self, req: Request) -> Response {
        let longest_chain_node = self.network.consensus_broadcast(self.blockchain.entity.len(), &self.network.endpoint);
        match longest_chain_node != self.network.endpoint {
            true => {
                let state = self.network.get_longest_node(&longest_chain_node);
                let mut nodes = vec![];
                let mut blocks = vec![];
                let mut transactions = vec![];
                self.network.nodes.clear();
                self.blockchain.entity.clear();
                self.blockchain.transactions.clear();
                // todo use macro
                for i in 0..state.nodes.len() {
                    nodes.push(state.nodes[i].clone());
                }
                for i in 0..state.blocks.len() {
                    blocks.push(state.blocks[i].clone());
                }
                for i in 0..state.transactions.len() {
                    transactions.push(state.transactions[i].clone());
                }
                self.network.nodes = nodes;
                self.blockchain.entity = blocks;
                self.blockchain.transactions = transactions;
                Response {
                    prefix: PREFIX.to_string(),
                    body: format!("sync with node listening on {}", longest_chain_node).to_string()
                }
            },
            false => {
                Response {
                    prefix: PREFIX.to_string(),
                    body: "this node chain is longest".to_string()
                }
            }
        }
    }

    pub fn add_block(&mut self, req: Request) -> Response {
        let block: Block = serde_json::from_str(&req.body).expect("fail to convert block to json");
        self.blockchain.entity.push(block.clone());
        self.blockchain.transactions.clear();
        Response {
            prefix: PREFIX.to_string(),
            body: json!(block).to_string()
        }
    }

    pub fn send_transaction(&mut self, req: Request) -> Response {
        let transaction: Transaction = serde_json::from_str(&req.body).expect("fail to convert transaction to json");
        // println!("{:?}", transaction);
        let transaction_json = json!(&transaction);
        self.blockchain.transactions.push(transaction.clone());
        // println!("{:?}", self.blockchain);
        Response {
            prefix: PREFIX.to_string(),
            body: transaction_json.to_string()
        }
    }

    pub fn add(&mut self, req: Request) -> Response {
        let body: Add = serde_json::from_str(&req.body).expect("fail to convert add to json");
        println!("add {} to network", body.endpoint);
        self.network.nodes.push(body.endpoint);
        Response {
            prefix: PREFIX.to_string(),
            body: "Ok".to_string()
        }
    }

    pub fn join(&mut self, req: Request) -> Response {
        let body: Add = serde_json::from_str(&req.body).expect("fail to convert add to json");
        println!("add {} to network", body.endpoint);
        let current_nodes = CurrentState {
            nodes: self.network.nodes.clone(),
            blocks: self.blockchain.entity.clone(),
            transactions: self.blockchain.transactions.clone()
        };
        self.network.broadcast(&body.endpoint);
        self.network.nodes.push(body.endpoint.clone());
        Response {
            prefix: PREFIX.to_string(),
            body: json!(current_nodes).to_string()
        }
    }

    pub fn get_state(&mut self, req: Request) -> Response {
        let current_nodes = CurrentState {
            nodes: self.network.nodes.clone(),
            blocks: self.blockchain.entity.clone(),
            transactions: self.blockchain.transactions.clone()
        };
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
