extern crate rand;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use rand::Rng;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

async fn peer_to_peer(req: Request<Body>) -> Result<Response<Body>, Infallible> {
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

fn random_port() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1024, 9000)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], random_port()));
    println!("{:?}", addr);
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(peer_to_peer))
    });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
