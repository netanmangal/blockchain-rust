use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use super::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub enum BlockStatus {
    Validated,
    Pending,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub creator_address: String,
    pub timestamp: DateTime,
    pub nonce: u32,
    pub status: BlockStatus,
    pub block_hash: String,
    pub prev_block_hash: String,
    pub merkle_root_hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn try_new(
        count: u32,
        creator_address: &String,
        nonce: u32,
        transactions: &Vec<Transaction>,
    ) -> Self {
        Block {
            index: count,
            creator_address: creator_address.clone(),
            timestamp: DateTime::now(),
            nonce: nonce,
            status: BlockStatus::Validated,
            block_hash: String::new(),
            prev_block_hash: String::new(),
            merkle_root_hash: String::new(),
            transactions: transactions.clone(),
        }
    }
}
