extern crate rand;
extern crate bigint;
extern crate rustc_hex;

mod key;

fn main() {
    let plain_text = "secret message";
    let private_key = key::PrivateKey::new();
    let signature = private_key.sign(&plain_text);
    let public_key = private_key.to_public_key();
    let is_valid = public_key.verify(plain_text, signature);
}
