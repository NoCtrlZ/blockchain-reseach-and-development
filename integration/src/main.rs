mod blockchain;
mod p2p;
mod unit;
mod debug;

extern crate rand;
use std::convert::Infallible;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};

pub async fn peer_to_peer(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new("hello world man".into())),
        (&Method::POST, "/check_all") => {
            println!("{:?}", req);
            Ok(Response::new("hello world man".into()))
        },
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], unit::random_port()));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(peer_to_peer))
    });
    let server = Server::bind(&addr).serve(make_svc);

    let mut blockchain = blockchain::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    let nonce = blockchain.proof_of_work();

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    debug::print_blockchain(blockchain);
}
