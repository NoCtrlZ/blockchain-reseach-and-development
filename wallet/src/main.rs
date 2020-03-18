extern crate rand;
extern crate bigint;
extern crate rustc_hex;

mod lamport;
use lamport::Wallet;

fn main() {
    let plain_text = "secret message";
    let wallet = Wallet::new();
    let signature = wallet.sign(&plain_text);
    let public_key = wallet.get_public_key();
    let is_valid = public_key.verify(plain_text, signature);
}
