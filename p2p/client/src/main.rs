use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use serde_json::json;
use serde::{Deserialize, Serialize};

const PREFIX: &str = "HTTP/1.1\r\nHost: localhost:5862\r\nUser-Agent: curl/7.64.1\r\nAccept: */*";

struct Request {
    method: String,
    path: String,
    body: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    amount: u64,
    sender: String,
    recipient: String
}

struct Add {
    endpoint: String
}

mod method {
    pub const GET: &str = "GET ";
    pub const POST: &str = "POST ";
}

fn request_contents(request: Request) -> String {
    let mut contents = request.method.to_string();
    contents.push_str(&format!("{}{}", request.path, " "));
    contents.push_str(PREFIX);
    contents.push_str(&format!("{}{}", "\r\n\r\n", request.body));
    contents
}

fn throw_request(endpoint: &str, request: Request) -> String {
    let mut stream = TcpStream::connect(endpoint).unwrap();
    stream.write(request_contents(request).as_bytes()).unwrap();
    let mut buffer = [0; 512];
    stream.read_exact(&mut buffer);
    String::from_utf8_lossy(&buffer[..]).trim_matches(char::from(0)).to_string()
}

fn get_blockchain(endpoint: &str) -> String {
    let request = Request {
        method: method::GET.to_string(),
        path: "/".to_string(),
        body: "".to_string()
    };
    throw_request(endpoint, request)
}

fn post_transaction(endpoint: &str, transaction: Transaction) -> String {
    let request = Request {
        method: method::POST.to_string(),
        path: "/send_transaction".to_string(),
        body: json!(transaction).to_string()
    };
    throw_request(endpoint, request)
}

fn send_transaction(endpoint: &str, amount: u64, sender: &str, recipient: &str) -> String {
    post_transaction(endpoint, Transaction {
        amount: amount,
        sender: sender.to_string(),
        recipient: recipient.to_string()
    })
}

fn create_new_block(endpoint: &str) -> String {
    let request = Request {
        method: method::GET.to_string(),
        path: "/create_new_block".to_string(),
        body: "".to_string()
    };
    throw_request(endpoint, request)
}

fn main() {
    let endpoint = "127.0.0.1:3000";
    let res1 = get_blockchain(endpoint);
    println!("{:?}", res1);
    let res2 = send_transaction(endpoint, 100, "Alice", "Bob");
    println!("{:?}", res2);
    let res3 = create_new_block(endpoint);
    println!("{:?}", res3);
}
