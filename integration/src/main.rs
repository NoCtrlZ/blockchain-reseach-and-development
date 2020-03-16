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
    router.post("/send_transaction", blockchain::Blockchain::send_transaction);
    router
}

fn main() {
    let router = set_router();
    server::Server::new(router).start();
    // blockchain.send_transaction(100, "alice", "bob");
    // let nonce = blockchain.proof_of_work();
}
