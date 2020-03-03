mod core;
mod unit;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut blockchain = core::Blockchain::new();
    blockchain.print_latest_block();
    blockchain.print_latest_block();
    blockchain.send_transaction(100, "alice", "bob");
    // latest_block.print_latest_transaction();
}
