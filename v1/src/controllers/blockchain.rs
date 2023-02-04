use std::usize;

use crate::models::block::Block;
use crate::utils::hasher::{self, hasher};
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::Database;
use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;
use rocket::{get, State};

#[get("/blockchain/get")]
pub async fn get_entire_blockchain(db: &State<Database>) -> Json<Vec<Block>> {
    let blocks = db
        .collection::<Block>("block")
        .find(None, FindOptions::builder().sort(doc! {"index": 1}).build())
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();

    Json(blocks)
}

#[get("/blockchain/isChainValid")]
pub async fn is_chain_valid(chain: &Vec<Block>) -> bool {
    for (i, block) in chain.iter().enumerate() {
        // skip genesis block
        if (i == 0) {
            continue;
        }

        // check prev block hash with current_block.prev_hash
        chain[i - 1].block_hash = block.prev_block_hash;

        // check block hash is valid
        let current_block_hash: &str = &block.block_hash;
        let b_hash: String = hasher(&[
            &block.index.to_string(),
            &block.creator_address,
            &block.timestamp.to_string(),
            &block.nonce.to_string(),
            &block.prev_block_hash,
            &block.merkle_root_hash,
        ]);

        if (b_hash != block.block_hash) {
            return false;
        }

        // check all txn_hashes are valid
        for (i, txn) in block.transactions.iter().enumerate() {
            let txn_hash: String = hasher(&[
                &txn.creator_address,
                &serde_json::to_string(&txn.transaction_type).unwrap(),
                &txn.timestamp.to_string(),
            ]);

            if (txn_hash != txn.transaction_hash) {
                return false;
            }
        }
    }

    return true;
}
