use std::io::Write;
use std::net::TcpStream;

fn main() {
    let message = "here is client";
    let endpoint = "127.0.0.1:3000";
    let stream = TcpStream::connect(endpoint);
    let mut stream = stream.unwrap();
    stream.write(message.as_bytes()).unwrap();
}
