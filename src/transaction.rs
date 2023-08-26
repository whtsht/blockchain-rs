use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: i32,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: i32) -> Self {
        Self {
            sender,
            recipient,
            amount,
        }
    }
}
