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
    router.post("/whole_blockchain", blockchain::Blockchain::check_all_handler);
    router
}

fn main() {
    let network = p2p::Network::new();
    let addr = network.get_address();
    let router = set_router();

    let mut blockchain = blockchain::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    let nonce = blockchain.proof_of_work();
    server::Server::new(router).start(&addr);
}
