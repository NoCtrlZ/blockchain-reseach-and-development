use std::io::Write;
use std::net::TcpStream;

pub const PREFIX: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: ";

pub struct Response {
    pub prefix: String,
    pub body: String
}

impl Response {
    pub fn write(&self, stream: &mut TcpStream) {
        // println!("response writer");
        let body = &self.body;
        let mut response = default_response(&self.prefix, &self.body.len());
        // println!("{:?}", response);
        response.push_str(&format!("{}{}", "\r\n\r\n", body));
        stream.write(response.as_bytes()).expect("fail to write bytes");
        stream.flush().expect("fail to flush stream");
    }
}

fn default_response(prefix: &str, contents_length: &usize) -> String {
    let mut response = prefix.to_string();
    response.push_str(&contents_length.to_string());
    response
}
