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
pub fn is_chain_valid(chain: &Vec<Block>) -> bool {
    for (i, block) in chain.iter().enumerate() {
        // skip genesis block
        if i == 0 {
            continue;
        }

        // check prev block hash with current_block.prev_hash
        if *chain[i - 1].block_hash != block.prev_block_hash {
            println!("\nPrev block hash not matching {}\n", i);
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
            println!("\nBlock has is not valid {}\n", i);
            return false;
        }
    }

    return true;
}

#[get("/blockchain/consensus")]
pub async fn perform_consensus(db: &State<Database>) -> String {
    let peers: Vec<Peer> = get_list_of_peers(db).await;
    let mut longest_chain: Vec<Block> = Vec::<Block>::new();

    let current_block_count: usize = db
        .collection::<Block>("block")
        .count_documents(None, None)
        .await
        .unwrap()
        .try_into()
        .unwrap();

    let reqwest_client = reqwest::Client::new();
    for p in peers.iter() {
        let incoming_chain: Vec<Block> = reqwest_client
            .get("http://".to_owned() + &p.address.clone() + "/blockchain/get")
            .send()
            .await
            .unwrap()
            .json::<Vec<Block>>()
            .await
            .unwrap();

        if is_chain_valid(&incoming_chain) {
            if incoming_chain.len() > longest_chain.len()
                && incoming_chain.len() > current_block_count
            {
                longest_chain = incoming_chain;
            }
        }
    }

    if longest_chain.len() > 1 {
        db.collection::<Block>("block").drop(None).await.ok();
        db.collection::<Block>("block")
            .insert_many(longest_chain, None)
            .await
            .ok();

        return format!("Chain has been updated.");
    } else {
        return format!("Chain has not been updated.");
    }
}
