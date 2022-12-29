use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

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
