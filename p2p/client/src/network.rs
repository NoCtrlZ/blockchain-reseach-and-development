use rand::seq::SliceRandom;

pub struct Network {
    host: String,
    nodes: Vec<Node>
}

struct Node {
    id: String,
    port: u16
}

impl Network {
    pub fn new() -> Network {
        let default_host = "127.0.0.1";
        let default_port = 3000;
        let mut network = Network {
            host: default_host.to_string(),
            nodes: Vec::new()
        };
        network.nodes.push(Node {
            id: random_string(),
            port: default_port
        });
        network
    }
}

fn random_string() -> String {
    // let base_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes();
    let sample = "Hello, audience!".as_bytes();
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        sample
            .choose_multiple(&mut rng, 16)
            .cloned()
            .collect()
    ).unwrap()
}