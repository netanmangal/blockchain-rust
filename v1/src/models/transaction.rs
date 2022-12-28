use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    Transfer(TransferTransaction)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Native(String),
    NonNative(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    creator_address: String,
    timestamp: DateTime,
    transaction_type: TransactionType,
    transaction_hash: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferTransaction {
    sender: String,
    receiver: String,
    amount: u32,
    token: TokenType
}