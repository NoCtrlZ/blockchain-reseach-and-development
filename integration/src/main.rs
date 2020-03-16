mod blockchain;
mod p2p;
mod unit;
mod debug;

extern crate rand;
use std::convert::Infallible;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};

#[tokio::main]
async fn main() {
    let network = p2p::Network::new();
    let addr = network.get_address();
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(p2p::peer_to_peer))
    });
    let server = Server::bind(&addr).serve(make_svc);

    let mut blockchain = blockchain::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    let nonce = blockchain.proof_of_work();

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    debug::print_blockchain(blockchain);
}
