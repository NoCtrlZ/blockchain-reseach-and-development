use std::io::Write;
use std::net::TcpStream;

const PREFIX: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: ";

struct Request {
    method: String,
    path: String,
    body: String
}

mod method {
    pub const GET: &str = "GET ";
    pub const POST: &str = "POST ";
}

fn create_normal_get(path: &str, body: &str) -> String {
    let mut request = method::GET.to_string();
    request.push_str(&format!("{}{}", path, " "));
    request.push_str(PREFIX);
    request.push_str(&body.len().to_string());
    request.push_str(&format!("{}{}", "\r\n\r\n", body));
    request
}

fn main() {
    let endpoint = "127.0.0.1:5862";
    let mut stream = TcpStream::connect(endpoint).unwrap();
    let res = stream.write(create_normal_get("/", "hello world").as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("{:?}", res);
}
