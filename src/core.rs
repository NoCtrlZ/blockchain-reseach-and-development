pub struct Blockchain<'a> {
    pub entity: Vec<Block<'a>>
}

pub struct Block<'a> {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction<'a>>,
    nonce: u64,
    hash: &'a str
}

pub struct Transaction<'a> {
    amount: u64,
    sender: &'a str,
    recipient: &'a str
}
