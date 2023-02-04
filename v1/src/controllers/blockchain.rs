use crate::controllers::peer::get_list_of_peers;
use crate::models::block::Block;
use crate::models::peer::Peer;
use crate::utils::hasher::hasher;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::Database;
use reqwest;
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

// #[get("/blockchain/isChainValid")]
/// this is the helper function
/// this doesn't needs to be route
pub async fn is_chain_valid(chain: &Vec<Block>) -> bool {
    for (i, block) in chain.iter().enumerate() {
        // skip genesis block
        if i == 0 {
            continue;
        }

        // check prev block hash with current_block.prev_hash
        if *chain[i - 1].block_hash != block.prev_block_hash {
            return false;
        }

        // check block hash is valid
        let b_hash: String = hasher(&[
            &block.index.to_string(),
            &block.creator_address,
            &block.timestamp.to_string(),
            &block.nonce.to_string(),
            &block.prev_block_hash,
            &block.merkle_root_hash,
        ]);

        if b_hash != block.block_hash {
            return false;
        }

        // check all txn_hashes are valid
        for txn in block.transactions.iter() {
            let txn_hash: String = hasher(&[
                &txn.creator_address,
                &serde_json::to_string(&txn.transaction_type).unwrap(),
                &txn.timestamp.to_string(),
            ]);

            if txn_hash != txn.transaction_hash {
                return false;
            }
        }
    }

    return true;
}

#[get("/consensus")]
pub async fn perform_consensus(db: &State<Database>) -> String {
    let peers: Vec<Peer> = get_list_of_peers(db).await;

    let reqwest_client = reqwest::Client::new();
    for p in peers.iter() {
        let incoming_blocks: Vec<Block> = reqwest_client
            .get("http://".to_owned() + &p.address.clone() + "/blockchain/get")
            .send()
            .await
            .unwrap()
            .json::<Vec<Block>>()
            .await
            .unwrap();

        println!("{:#?}", incoming_blocks);
    }

    // return format!("Chain has been updated from {}", peer);
    return format!("Chain has not been updated.");
}
