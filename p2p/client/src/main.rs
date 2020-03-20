use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

const PREFIX: &str = "HTTP/1.1\r\nHost: localhost:5862\r\nUser-Agent: curl/7.64.1\r\nAccept: */*";

struct Request {
    method: String,
    path: String,
    body: String
}

mod method {
    pub const GET: &str = "GET ";
    pub const POST: &str = "POST ";
}

fn create_normal_get(path: &str, body: &str) -> String {
    let mut request = method::GET.to_string();
    request.push_str(&format!("{}{}", path, " "));
    request.push_str(PREFIX);
    // request.push_str(&body.len().to_string());
    request.push_str("\r\n\r\n");
    // request.push_str(&format!("{}{}", "\r\n\r\n", body));
    request
}

fn main() {
    let endpoint = "127.0.0.1:3000";
    let mut stream = TcpStream::connect(endpoint).unwrap();
    stream.write(create_normal_get("/", "hello world").as_bytes()).unwrap();
    let mut buffer = [0; 512];
    stream.read_exact(&mut buffer);
    println!("{:?}", String::from_utf8_lossy(&buffer[..]).trim_matches(char::from(0)).to_string());
}
