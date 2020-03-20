use bigint::U256;
use serde::{Deserialize, Serialize};
use crate::lamport::PublicKey;

#[derive(Debug)]
pub struct Transfer {
    pub sender: String,
    pub public_key: PublicKey,
    pub transaction: Transaction,
    pub verification: Verification
}

#[derive(Debug)]
pub struct Issue {
    pub sender: String,
    pub public_key: PublicKey,
    pub certificate: Certificate,
    pub verification: Verification
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub amount: u64,
    pub sender: String,
    pub recipient: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Certificate {
    pub id: String,
    pub owner: String,
    pub terms: u8,
    pub royalty: u8,
    pub content_hash: String,
}

#[derive(Debug)]
pub struct Verification {
    pub message: Vec<U256>,
    pub tx_hash: String
}
