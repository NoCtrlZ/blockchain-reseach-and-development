mod blockchain;
mod p2p;
mod unit;
mod debug;
mod request;
mod router;
mod server;
mod response;
mod lamport;
mod utxo;
mod base64;

extern crate rand;
extern crate bigint;
extern crate rustc_hex;

use server::Server;
use router::Router;

fn set_router() -> Router {
    let mut router = Router::new();
    router.get("/", Server::get_all);
    router.get("/nodes", Server::get_nodes);
    router.get("/check", Server::get_node_info);
    router.get("/balance", Server::balance);
    router.get("/network", Server::get_network);
    router.get("/blockchain", Server::get_blockchain);
    router.get("/create_new_block", Server::create_new_block);
    router.get("/consensus", Server::consensus);
    router.get("/get_state", Server::get_state);
    router.post("/add", Server::add);
    router.post("/join", Server::join);
    router.post("/add_block", Server::add_block);
    router.post("/send_transaction", Server::send_transaction);
    router
}

fn main() {
    let router = set_router();
    Server::new(router).start();
}
