extern crate rand;
use rand::Rng;
mod request;
mod router;
mod server;

fn random_port() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1024, 9000)
}

fn main() {
    let mut router = router::Router::new();
    router.get("/", request::Request::index_handler);
    server::Server::new(router).start("127.0.0.1:5000")
}
