use std::net::TcpListener;

pub fn main() {
    let endpoint = "127.0.0.1:9000";
    let listener = TcpListener::bind(&endpoint).unwrap();
    for stream in listener.incoming() {
        println!("Server is listening on {}", endpoint);
        println!("{:?}", stream);
    }
}
