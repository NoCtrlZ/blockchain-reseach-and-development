extern crate rand;
extern crate bigint;

mod key;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_private_key_new() {
        let key = key::PrivateKey::new();
        println!("hello");
        println!("{:?}", key);
    }
}
