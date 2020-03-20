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
        // println!("response writer");
        let body = &self.body;
        let contents_length = &self.body.len().to_string();
        // println!("{:?}", response);
        let mut response = prefix::PREFIX.to_string();
        response.push_str(contents_length);
        response.push_str(&format!("{}{}", "\r\n\r\n", body));
        stream
            .write(response.as_bytes())
            .unwrap();
        stream.flush().unwrap();
    }
}
