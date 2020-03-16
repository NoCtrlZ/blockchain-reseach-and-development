extern crate rand;
use std::convert::Infallible;
use hyper::{Body, Method, Request, Response, StatusCode};

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
