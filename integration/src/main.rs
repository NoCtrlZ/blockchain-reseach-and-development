mod blockchain;
mod p2p;
mod unit;
mod debug;
mod request;
mod router;
mod server;
mod response;

fn set_router() -> router::Router {
    let mut router = router::Router::new();
    router.get("/", blockchain::Blockchain::index_handler);
    router.get("/whole_blockchain", blockchain::Blockchain::check_all_handler);
    router
}

fn main() {
    let router = set_router();
    server::Server::new(router).start();
    let network = p2p::Network::new();
    let addr = network.get_address();
    let mut blockchain = blockchain::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    let nonce = blockchain.proof_of_work();
}
