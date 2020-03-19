extern crate rand;
mod request;
mod router;
mod network;
mod response;

use request::Request;
use router::Router;
use network::Network;

fn main() {
    let mut router = Router::new();
    router.get("/", Network::get_nodes);
    router.post("/add", Network::add);
    Network::new(router)
}
