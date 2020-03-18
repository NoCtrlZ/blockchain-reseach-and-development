extern crate rand;
extern crate bigint;
extern crate rustc_hex;

mod key;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_private_key_new() {
        let plain_text = "secret message";
        let private_key = key::PrivateKey::new();
        let signature = private_key.sign(&plain_text);
        let public_key = private_key.to_public_key();
        let is_valid = public_key.verify(plain_text, signature);
        assert_eq!(key::PRIVATE_KEY_LENGT, private_key.pairs.len());
        assert_eq!(key::PRIVATE_KEY_LENGT, private_key.public_key.pairs.len());
        assert_eq!(true, is_valid);
    }
}
