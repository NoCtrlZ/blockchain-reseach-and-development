mod blockchain;
mod p2p;
mod unit;
mod debug;
mod request;
mod router;
mod server;
mod response;

use server::Server;

fn set_router() -> router::Router {
    let mut router = router::Router::new();
    router.get("/", Server::index_handler);
    router.get("/whole_blockchain", Server::check_all_handler);
    router.post("/send_transaction", Server::send_transaction);
    router
}

fn main() {
    let router = set_router();
    Server::new(router).start();
}
