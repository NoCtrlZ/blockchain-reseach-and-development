use std::net::TcpStream;

fn main() {
    let endpoint = "127.0.0.1:9000";
    let stream = TcpStream::connect(endpoint);
}
