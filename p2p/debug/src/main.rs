use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    stream.write(b"NICK G-SERUFU\r\n");

    let mut line = String::with_capacity(512);
    let result = stream.read_to_string(&mut line);
    match result {
        Ok(n) => println!("Received {} bytes", n),
        _ => {}
    }
    line.clear();
}
