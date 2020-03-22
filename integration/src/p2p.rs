use serde_json::json;
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

const PREFIX: &str = "HTTP/1.1\r\nHost: localhost:5862\r\nUser-Agent: curl/7.64.1\r\nAccept: */*";

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

mod method {
    pub const GET: &str = "GET ";
    pub const POST: &str = "POST ";
}

impl Network {
    pub fn new() -> Network {
        let original_endpoint = "127.0.0.1:3000";
        let mut endpoint = "127.0.0.1:".to_string();
        if default_port_is_open() {
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
    let mut buffer = [0; 512];
    stream.read_exact(&mut buffer);
    String::from_utf8_lossy(&buffer[..]).trim_matches(char::from(0)).to_string()
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
