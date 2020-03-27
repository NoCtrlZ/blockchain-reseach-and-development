use serde_json::json;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::io::prelude::*;
use crate::blockchain::Block;
use crate::request::Request;
use crate::unit::{is_open, random_port};
use crate::utxo::Transaction;
use crate::server::NodeInfo;

const PREFIX: &str = "HTTP/1.1\r\nHost: localhost:5862\r\nUser-Agent: curl/7.64.1\r\nAccept: */*";

#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    pub endpoint: String,
    pub nodes: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkInfo {
    pub endpoint: String,
    pub nodes: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct Throw {
    method: String,
    path: String,
    body: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Add {
    pub endpoint: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentState {
    pub nodes: Vec<String>,
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>
}

mod method {
    pub const GET: &str = "GET ";
    pub const POST: &str = "POST ";
}

impl Network {
    pub fn new() -> (Network, Vec<Block>, Vec<Transaction>) {
        let original_endpoint = "127.0.0.1:3000";
        let mut endpoint = "127.0.0.1:".to_string();
        if is_open(&original_endpoint) {
            println!("default port is open");
            let port = random_port();
            endpoint.push_str(&port);
            let res = join_network(&original_endpoint, &endpoint);
            let mut body: CurrentState = serde_json::from_str(&res.body).expect("fail to pase current node to json");
            // println!("{:?}", body);
            body.nodes.push(original_endpoint.to_string());
            println!("I am node listening on {}!", &port);
            return (Network {
                endpoint: endpoint,
                nodes: body.nodes
            }, body.blocks.clone(), body.transactions.clone());
        } else {
            endpoint.push_str("3000");
            println!("I am original node!");
            return (Network {
                endpoint: endpoint,
                nodes: Vec::new()
            }, Vec::new(), Vec::new());
        }
    }

    pub fn broadcast(&mut self, node: &str) {
        for i in 0..self.nodes.len() {
            add_node_to_network(&self.nodes[i], node);
        }
    }

    pub fn block_broadcast(&mut self, block: Block) {
        for i in 0..self.nodes.len() {
            add_block_to_blockchain(&self.nodes[i], block.clone());
        }
    }

    pub fn transaction_broadcast(&mut self, transaction: Transaction) {
        for i in 0..self.nodes.len() {
            add_transaction_to_blockchain(&self.nodes[i], transaction.clone());
        }
    }

    pub fn consensus_broadcast(&self, length: usize, endpoint: &str) -> String {
        let mut longest_chain_node = endpoint.to_string();
        let mut max_block_length = length;
        for i in 0..self.nodes.len() {
            let (length, endpoint) = get_chain_length(&self.nodes[i]);
            if length > max_block_length {
                longest_chain_node = endpoint.clone();
                max_block_length = length;
            }
        }
        longest_chain_node.to_string()
    }

    pub fn get_longest_node(&self, endpoint: &str) -> CurrentState {
        let res = request_state(&endpoint);
        serde_json::from_str(&res.body).expect("fail to pase current state to json")
    }

    pub fn network_json(&self) -> String {
        let network = json!(&self);
        network.to_string()
    }
}

fn request_contents(request: Throw) -> String {
    let mut contents = request.method.to_string();
    contents.push_str(&format!("{}{}", request.path, " "));
    contents.push_str(PREFIX);
    contents.push_str(&format!("{}{}", "\r\n\r\n", request.body));
    contents
}

fn throw_request(endpoint: &str, request: Throw) -> Request {
    let mut stream = TcpStream::connect(endpoint).expect("fail to connect tcp stream");
    stream.write(request_contents(request).as_bytes()).expect("fail to write response");
    Request::parse(&mut stream)
}

fn post_node(endpoint: &str, node: Add) -> Request {
    let request = Throw {
        method: method::POST.to_string(),
        path: "/add".to_string(),
        body: json!(node).to_string()
    };
    throw_request(endpoint, request)
}

fn add_node_to_network(endpoint: &str, node: &str) -> Request {
    post_node(endpoint, Add {
        endpoint: node.to_string()
    })
}

fn add_block_to_blockchain(endpoint: &str, block: Block) -> Request {
    let request = Throw {
        method: method::POST.to_string(),
        path: "/add_block".to_string(),
        body: json!(block).to_string()
    };
    throw_request(endpoint, request)
}

fn add_node(endpoint: &str, node: Add) -> Request {
    let request = Throw {
        method: method::POST.to_string(),
        path: "/join".to_string(),
        body: json!(node).to_string()
    };
    throw_request(endpoint, request)
}

fn join_network(endpoint: &str, node: &str) -> Request {
    add_node(endpoint, Add {
        endpoint: node.to_string()
    })
}

fn add_transaction_to_blockchain(endpoint: &str, transaction: Transaction) -> Request {
    let request = Throw {
        method: method::POST.to_string(),
        path: "/send_transaction".to_string(),
        body: json!(transaction).to_string()
    };
    throw_request(endpoint, request)
}

fn get_chain_length(endpoint: &str) -> (usize, String) {
    let request = Throw {
        method: method::GET.to_string(),
        path: "/check".to_string(),
        body: "".to_string()
    };
    let request = throw_request(endpoint, request);
    let body: NodeInfo = serde_json::from_str(&request.body).expect("fail to pase node info to json");
    (body.block_length, body.endpoint.to_string())
}

pub fn request_state(endpoint: &str) -> Request {
    let request = Throw {
        method: method::GET.to_string(),
        path: "/get_state".to_string(),
        body: "".to_string()
    };
    throw_request(endpoint, request)
}