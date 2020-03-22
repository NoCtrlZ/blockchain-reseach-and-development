mod utxo;

use utxo::Transaction;

fn main() {
    let dummy_address = "0x114514";
    let mut utxo = Transaction::new();
    utxo.admin_transfer(dummy_address);
    println!("{:?}", utxo);
}
