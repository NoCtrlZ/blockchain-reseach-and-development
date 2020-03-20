extern crate rand;
extern crate bigint;
extern crate rustc_hex;
extern crate base64;

mod lamport;
mod transaction;
use lamport::{Wallet, sha256};
use transaction::*;
use serde_json::json;

fn main() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();
    let public_key1 = wallet1.get_public_key();
    let address1 = wallet1.get_address();
    let transaction = Transaction {
        amount: 100,
        sender: wallet1.get_address().to_string(),
        recipient: wallet2.get_address().to_string()
    };
    let tx_hash = sha256(&json!(&transaction).to_string());
    let message = wallet1.sign(&tx_hash);
    let verification = Verification {
        message: message,
        tx_hash: tx_hash
    };
    let transfer = Transfer {
        sender: address1,
        public_key: public_key1.clone(),
        transaction: transaction,
        verification: verification
    };
    let valid = public_key1.verify(&transfer.verification.tx_hash, transfer.verification.message);
    let altered = sha256(&json!(&transfer.transaction).to_string()) != transfer.verification.tx_hash;
    if !valid || altered {
        panic!("transaction is invalid");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_test() {
        let wallet1 = Wallet::new();
        let wallet2 = Wallet::new();
        let public_key1 = wallet1.get_public_key();
        let address1 = wallet1.get_address();
        let transaction = Transaction {
            amount: 100,
            sender: wallet1.get_address().to_string(),
            recipient: wallet2.get_address().to_string()
        };
        let tx_hash = sha256(&json!(&transaction).to_string());
        let message = wallet1.sign(&tx_hash);
        let verification = Verification {
            message: message,
            tx_hash: tx_hash
        };
        let transfer = Transfer {
            sender: address1,
            public_key: public_key1.clone(),
            transaction: transaction,
            verification: verification
        };
        let valid = public_key1.verify(&transfer.verification.tx_hash, transfer.verification.message);
        let altered = sha256(&json!(&transfer.transaction).to_string()) != transfer.verification.tx_hash;
        assert_eq!(true, valid);
        assert_eq!(false, altered);
    }
}
