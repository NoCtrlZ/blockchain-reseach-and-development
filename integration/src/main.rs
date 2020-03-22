mod blockchain;
mod p2p;
mod unit;
mod debug;
mod request;
mod router;
mod server;
mod response;
mod lamport;

extern crate rand;
extern crate bigint;
extern crate rustc_hex;
extern crate base64;

use server::Server;
use router::Router;

fn set_router() -> Router {
    let mut router = Router::new();
    router.get("/", Server::get_blockchain);
    router.get("/create_new_block", Server::create_new_block);
    router.post("/send_transaction", Server::send_transaction);
    router
}

fn main() {
    let router = set_router();
    Server::new(router).start();
}
