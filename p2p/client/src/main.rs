mod network;
extern crate rand;
use std::net::TcpStream;

fn main() {
    let network = network::Network::new();
}
