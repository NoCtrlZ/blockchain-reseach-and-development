mod blockchain;
mod p2p;
mod unit;
mod debug;
mod request;
mod router;
mod server;
mod response;

fn main() {
    let network = p2p::Network::new();
    let addr = network.get_address();
    let mut router = router::Router::new();
    router.get("/", request::Request::index_handler);
    println!("{:?}", &addr);

    let mut blockchain = blockchain::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    let nonce = blockchain.proof_of_work();
    server::Server::new(router).start(&addr);
}
