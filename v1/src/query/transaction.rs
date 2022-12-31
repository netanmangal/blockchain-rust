use serde::{Serialize, Deserialize};

use crate::models::transaction::TransactionType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionInputData {
    pub creator_address: String,
    pub transaction_type: TransactionType,
    pub transaction_hash: String,
}