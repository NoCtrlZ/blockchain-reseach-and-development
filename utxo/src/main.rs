mod utxo;

use utxo::Transaction;

fn main() {
    let dummy_address = "0x114514";
    let mut utxo = Transaction::new();
    let tx_hash = utxo.admin_transfer(dummy_address);
    println!("tx hash is {}", &tx_hash);
    println!("{:?}", utxo);
}
