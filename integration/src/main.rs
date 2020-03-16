mod blockchain;
mod p2p;
mod unit;

use std::net::SocketAddr;
use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], unit::random_port()));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(p2p::peer_to_peer))
    });
    let server = Server::bind(&addr).serve(make_svc);

    let mut blockchain = blockchain::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    let nonce = blockchain.proof_of_work();
    blockchain.print_blockchain();
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
