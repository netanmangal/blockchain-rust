use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};

use super::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub enum BlockStatus {
    Validated,
    Pending,
    Rejected
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    index: u32,
    creator_address: String,
    timestamp: DateTime,
    nonce: u32,
    status: BlockStatus,
    block_hash: String,
    prev_block_hash: String,
    merkle_root_hash: String,
    transactions: Vec<Transaction>
}