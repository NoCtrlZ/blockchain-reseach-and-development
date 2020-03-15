mod core;
mod unit;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut blockchain = core::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    blockchain.send_transaction(200, "alice", "bob");
    let nonce = blockchain.proof_of_work();
    blockchain.send_transaction(100, "bod", "alice");
    blockchain.print_blockchain();
}
