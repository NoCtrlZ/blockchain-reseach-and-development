#[derive(Debug)]
pub struct Network {
    pub nodes: Vec<Node>
}

#[derive(Debug)]
pub struct Node {
    port: u16
}

impl Network {
    pub fn new(self, port: u16) -> Network {
        let mut network = Network {
            nodes: Vec::new()
        };
        let original_node = Node {
            port: port
        };
        network.nodes.push(original_node);
        network
    }
}
