mod core;
mod unit;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut blockchain = core::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    blockchain.send_transaction(100, "alice", "bob");
    blockchain.proof_of_work();
    // println!("{:?}", hash);
    blockchain.print_blockchain();
}
