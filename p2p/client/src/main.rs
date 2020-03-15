use hyper::{body::HttpBody as _, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let uri = "http://localhost:3000".parse().unwrap();
    let resp = client.get(uri).await.unwrap();
    println!("Response: {}", resp.status());
}
