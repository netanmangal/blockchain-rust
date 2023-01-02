use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use super::transaction::Transaction;
use crate::utils::hasher;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BlockStatus {
    Validated,
    Pending,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        prev_block_hash: &String,
    ) -> Self {
        let timestamp: DateTime = DateTime::now();
        let merkle_root_hash: String = String::new(); // change

        let block_hash: String = hasher::hasher(&[
            &count.to_string(),
            creator_address,
            &timestamp.to_string(),
            &nonce.to_string(),
            prev_block_hash,
            &merkle_root_hash,
        ]);

        Block {
            index: count,
            creator_address: creator_address.clone(),
            timestamp: timestamp,
            nonce: nonce,
            status: BlockStatus::Validated,
            prev_block_hash: prev_block_hash.clone(),
            merkle_root_hash: merkle_root_hash,
            block_hash: block_hash,
            transactions: transactions.clone(),
        }
    }
}
