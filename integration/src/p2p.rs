#[derive(Debug)]
pub struct Network {
    pub nodes: Vec<Node>,
    pub host: [i32; 4]
}

#[derive(Debug)]
pub struct Node {
    pub port: u16
}

impl Network {
    pub fn new() -> Network {
        let mut network = Network {
            nodes: Vec::new(),
            host: [127, 0, 0, 1],
        };
        let original_node = Node {
            port: 3000
        };
        network.nodes.push(original_node);
        network
    }

    pub fn get_address(&self) -> String {
        let mut address = "".to_string();
        for i in 0..self.host.len() {
            address.push_str(&self.host[i].to_string());
            if i != self.host.len() - 1 {
                address.push_str(".");
            } else {
                address.push_str(":");
                address.push_str(&self.nodes[0].port.to_string());
            }
        }
        address
    }
}
