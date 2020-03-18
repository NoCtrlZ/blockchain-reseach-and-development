#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transfer {
    amount: u64,
    sender: String,
    recipient: String,
    verification: Verification,
}

pub struct Issue {
    sender: String,
    certificate: Certificate,
    verification: Verification
}

pub struct Certificate {
    id: String,
    owner: String,
    terms: u8,
    royalty: u8,
    content_hash: String,
}

pub struct Verification {
    message: String,
    tx_hash: String
}
