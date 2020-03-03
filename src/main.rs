mod core;
use std::any::type_name;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut sha256 = Sha256::new();
    sha256.input_str("Hi Sexy");
    println!("{:?}", type_of(&sha256.result_str()));
}
