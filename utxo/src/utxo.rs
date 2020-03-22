#[derive(Debug)]
pub struct Transaction {
    input: Box<Option<Input>>,
    output: Box<Option<Output>>
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
            input: Box::new(None),
            output: Box::new(None)
        }
    }
}
