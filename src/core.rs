use std::time::SystemTime;

pub struct Blockchain {
    pub entity: Vec<Block>
}

struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    nonce: u64
}

