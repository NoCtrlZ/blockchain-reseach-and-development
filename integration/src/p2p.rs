use serde_json::json;
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use crate::blockchain::Block;
use crate::response::Response;
use crate::request::Request;
use crate::unit::{is_open, random_port, stream_to_string};

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

mod method {
    pub const GET: &str = "GET ";
    pub const POST: &str = "POST ";
}

impl Network {
    pub fn new() -> Network {
        let original_endpoint = "127.0.0.1:3000";
        let mut endpoint = "127.0.0.1:".to_string();
        if is_open(&original_endpoint) {
            println!("default port is open");
            let port = random_port();
            endpoint.push_str(&port);
            add_node_to_network(&original_endpoint, &endpoint);
            println!("I am node listening on {}!", &port);
        } else {
            endpoint.push_str("3000");
            println!("I am original node!");
        }
        Network {
            endpoint: endpoint,
            nodes: Vec::new()
        }
    }

    pub fn broadcast(&mut self, node: &str) {
        for i in 0..self.nodes.len() {
            add_node_to_network(&self.nodes[i], node);
        }
    }

    pub fn block_broadcast(&mut self, block: Block) {
        for i in 0..self.nodes.len() {

        }
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

fn throw_request(endpoint: &str, request: Throw) -> String {
    let mut stream = TcpStream::connect(endpoint).unwrap();
    stream.write(request_contents(request).as_bytes()).unwrap();
    stream_to_string(stream)
}

fn post_node(endpoint: &str, node: Add) -> String {
    let request = Throw {
        method: method::POST.to_string(),
        path: "/add".to_string(),
        body: json!(node).to_string()
    };
    throw_request(endpoint, request)
}

fn add_node_to_network(endpoint: &str, node: &str) -> String {
    post_node(endpoint, Add {
        endpoint: node.to_string()
    })
}

fn add_block_to_blockchain(endpoint: &str, block: Block) -> String {
    let request = Throw {
        method: method::POST.to_string(),
        path: "/add_block".to_string(),
        body: json!(block).to_string()
    };
    throw_request(endpoint, request)
}
