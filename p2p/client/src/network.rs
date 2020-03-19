use rand::seq::SliceRandom;
use std::net::{TcpListener, TcpStream};
use rand::Rng;

pub struct Network {
    endpoint: String,
    nodes: Vec<String>
}

impl Network {
    pub fn new() {
        let mut default_node = "127.0.0.1:".to_string();
        default_node.push_str(&random_port());
        let mut network = Network {
            endpoint: default_node,
            nodes: Vec::new()
        };
        let listener = TcpListener::bind(&network.endpoint).unwrap();
        for stream in listener.incoming() {
            let response = network.handle(&mut stream.unwrap());
        }
    }

    fn handle(&self, stream: &mut TcpStream) {
        println!("{:?}", stream);
    }
}

pub fn random_port() -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(1024, 9000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;

    #[test]
    fn test_get_endpoint() {
        let network = Network::new();
        let endpoint = network.endpoint();
        assert_eq!("127.0.0.1:3000", endpoint);
    }

    #[test]
    fn test_random_string() {
        let random_string = random_string();
        assert_eq!(16, random_string.len());
        assert_eq!(type_of(&"str".to_string()), type_of(&random_string));
    }

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
}
