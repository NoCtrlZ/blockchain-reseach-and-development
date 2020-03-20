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

fn get_root_request(endpoint: &str) -> String {
    let request = Request {
        method: method::GET.to_string(),
        path: "/".to_string(),
        body: "hello world".to_string()
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

fn main() {
    let endpoint = "127.0.0.1:3000";
    let res = get_root_request(endpoint);
    // println!("{:?}", res);
    let transaction = Transaction {
        amount: 100,
        sender: "Alice".to_string(),
        recipient: "Bob".to_string()
    };
    let result = post_transaction(endpoint, transaction);
    println!("{:?}", result);
}
