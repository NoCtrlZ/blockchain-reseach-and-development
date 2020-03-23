mod utxo;

use utxo::Utxo;

fn main() {
    let dummy_address = "0x114514";
    let mut utxo = Utxo::new();
    let tx_hash = utxo.admin_transfer(dummy_address);
    println!("tx hash is {}", &tx_hash);
    println!("{:?}", utxo);
}
