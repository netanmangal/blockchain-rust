use mongodb::bson::{self, doc, DateTime};
use mongodb::options::FindOneOptions;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};

use crate::models::block::Block;
use crate::query::block::CreateBlockInputData;
use crate::utils::pow;

#[get("/block/get?<index>")]
pub async fn get_block_by_index(index: u32, db: &State<Database>) -> Json<Block> {
    let block: Block = db
        .collection::<Block>("block")
        .find_one(
            doc! {
                "index": index
            },
            None,
        )
        .await
        .unwrap()
        .expect("Missing block with given index.");

    return Json(block);
}

#[post("/block/mine", format = "application/json", data = "<block_data>")]
pub async fn mine_new_block(
    block_data: Json<CreateBlockInputData>,
    db: &State<Database>,
) -> Json<Block> {
    let count: u32 = db
        .collection::<Block>("block")
        .count_documents(None, None)
        .await
        .unwrap()
        .try_into() // convert u64 into u32
        .unwrap();

    let mut prev_block_hash: String = String::new();
    if count != 0 {
        let prev_block: Block = db
            .collection::<Block>("block")
            .find_one(
                doc! {
                    "index": count - 1
                },
                None,
            )
            .await
            .unwrap()
            .unwrap();

        prev_block_hash = prev_block.block_hash;
    }

    let timestamp: DateTime = DateTime::now();
    let merkle_root_hash = String::new();

    let nonce: u32 = pow::proof_of_work(
        &count.to_string(),
        &block_data.creator_address,
        &timestamp.to_string(),
        &prev_block_hash,
        &merkle_root_hash,
    );

    return create_new_block(
        Json(Block::try_new(
            count,
            &block_data.creator_address,
            timestamp,
            nonce,
            &block_data.transactions,
            &prev_block_hash,
        )),
        db,
    )
    .await;
}

async fn create_new_block(block: Json<Block>, db: &State<Database>) -> Json<Block> {
    db.collection::<Block>("block")
        .insert_one(block.clone().into_inner(), None)
        .await
        .ok();

    return block;
}

#[get("/block/last")]
pub async fn get_last_block(db: &State<Database>) -> Json<Block> {
    let block: Block = db
        .collection::<Block>("block")
        .find_one(
            None,
            FindOneOptions::builder().sort(doc! {"index": -1}).build(),
        )
        .await
        .unwrap()
        .unwrap();

    Json(block)
}

#[put("/block/update", format = "application/json", data = "<updated_block>")]
pub async fn update_block(updated_block: Json<Block>, db: &State<Database>) -> String {
    let new_block: Block = Block::try_new(
        updated_block.index,
        &updated_block.creator_address,
        updated_block.timestamp,
        updated_block.nonce,
        &updated_block.transactions,
        &updated_block.prev_block_hash,
    );

    db.collection::<Block>("block")
        .update_one(
            doc! {
                "index": updated_block.index
            },
            doc! {
                "$set": bson::to_bson( &new_block ).unwrap()
            },
            None,
        )
        .await
        .ok();

    format!("Block has been updated successfully!!!")
}

#[delete("/block/delete?<index>")]
pub async fn delete_block(index: u32, db: &State<Database>) -> String {
    db.collection::<Block>("block")
        .delete_one(
            doc! {
                "index": index
            },
            None,
        )
        .await
        .ok();

    format!("Block has been deleted successfully!!!")
}

#[get("/block/count")]
pub async fn get_block_count(db: &State<Database>) -> String {
    db.collection::<Block>("block")
        .count_documents(None, None)
        .await
        .unwrap()
        .to_string()
}
