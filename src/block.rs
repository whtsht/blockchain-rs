use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::time;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String,
}

impl Block {
    pub fn new(
        index: u32,
        transactions: Vec<Transaction>,
        proof: u64,
        previous_hash: String,
    ) -> Self {
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            index,
            timestamp,
            transactions,
            proof,
            previous_hash,
        }
    }
}
