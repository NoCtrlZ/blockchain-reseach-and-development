use std::io::Write;
use std::net::TcpStream;

pub mod prefix {
    pub const PREFIX: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: ";
}

pub struct Response {
    pub prefix: String,
    pub body: String
}

impl Response {
    pub fn write(&self, stream: &mut TcpStream) {
        let response = &self.body;
        let hello = "hello";
        stream
            .write(format!("{}{}", hello, response).as_bytes())
            .unwrap();
        stream.flush().unwrap();
    }
}
