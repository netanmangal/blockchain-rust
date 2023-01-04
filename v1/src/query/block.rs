use serde::{Serialize, Deserialize};

use crate::models::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlockInputData {
    pub creator_address: String,
    pub transactions: Vec<Transaction>
}