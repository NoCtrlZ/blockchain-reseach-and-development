use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn read_stream(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer[..])
        .trim_matches(char::from(0))
        .to_string()
}

pub fn main() {
    let endpoint = "127.0.0.1:9000";
    let listener = TcpListener::bind(&endpoint).unwrap();
    for stream in listener.incoming() {
        println!("Server is listening on {}", endpoint);
        let message = read_stream(&mut stream.unwrap());
        println!("{:?}", message);
    }
}
