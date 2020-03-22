#[derive(Debug)]
pub struct Transaction {
    input: Vec<Input>,
    output: Vec<Output>
}

#[derive(Debug)]
struct Input {
    transaction: Transaction,
    output_index: u8
}

#[derive(Debug)]
struct Output {
    recipient: String,
    amount: u64
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            input: Vec::new(),
            output: Vec::new()
        }
    }

    pub fn admin_transfer(&mut self, address: &str) {
        let output = Output {
            recipient: address.to_string(),
            amount: 100
        };
        self.output.push(output);
    }
}
