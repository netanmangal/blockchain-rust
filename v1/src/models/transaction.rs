use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::utils::hasher;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Transfer(TransferTransaction),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TokenType {
    Native(String),
    NonNative(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub creator_address: String,
    pub timestamp: DateTime,
    pub transaction_type: TransactionType,
    pub transaction_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferTransaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u32,
    pub token: TokenType,
}

impl Transaction {
    pub fn try_new(creator_address: &String, transaction_type: &TransactionType) -> Self {
        let timestamp: DateTime = DateTime::now();

        let transaction_hash = hasher::hasher(&[
            &creator_address,
            &serde_json::to_string(&transaction_type).unwrap(),
            &timestamp.to_string(),
        ]);

        Transaction {
            creator_address: creator_address.clone(),
            timestamp: timestamp,
            transaction_type: transaction_type.clone(),
            transaction_hash: transaction_hash,
        }
    }
}
