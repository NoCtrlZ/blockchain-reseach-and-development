use serde_json::json;
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::net::TcpListener;

#[derive(Debug)]
pub struct Network {
    pub endpoint: String,
    nodes: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkInfo {
    endpoint: String,
    nodes: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct Throw {
    method: String,
    path: String,
    body: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Add {
    endpoint: String
}

impl Network {
    pub fn new() -> Network {
        let original_endpoint = "127.0.0.1:3000";
        let mut endpoint = "127.0.0.1:".to_string();
        if default_port_is_open() {
            // println!("default port is open");
            // let port = random_port();
            // endpoint.push_str(&port);
            // add_node_to_network(&original_endpoint, &endpoint);
            // println!("I am node listening on {}!", &port);
        } else {
            endpoint.push_str("3000");
            println!("I am original node!");
        }
        Network {
            endpoint: endpoint,
            nodes: Vec::new()
        }
    }
}

fn random_port() -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(1025, 9000).to_string()
}

fn default_port_is_open() -> bool {
    match TcpListener::bind(("127.0.0.1", 3000)) {
        Ok(_) => false,
        Err(_) => true,
    }
}
