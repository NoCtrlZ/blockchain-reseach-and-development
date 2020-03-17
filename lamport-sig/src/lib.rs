extern crate rand;
extern crate bigint;

mod key;

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_private_key_new() {
        let key = key::PrivateKey::new();
        assert_eq!(key::PRIVATE_KEY_LENGT, key.pairs.len());
        assert_eq!(key::PRIVATE_KEY_LENGT, key.public_key.pairs.len());
    }
}
