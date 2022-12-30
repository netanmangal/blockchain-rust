use mongodb::bson::{self, doc};
use mongodb::options::FindOneOptions;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};

use crate::models::block::Block;
use crate::query::block::CreateBlockInputData;

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

#[post("/block/create", format = "application/json", data = "<block_data>")]
pub async fn create_new_block(
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

    let new_block: Block = Block::try_new(
        count,
        &block_data.creator_address,
        block_data.nonce,
        &block_data.transactions,
    );

    db.collection::<Block>("block")
        .insert_one(&new_block, None)
        .await
        .ok();

    return Json(new_block);
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

#[put("/block/update", format = "application/json", data = "<new_block>")]
pub async fn update_block(new_block: Json<Block>, db: &State<Database>) -> String {
    db.collection::<Block>("block")
        .update_one(
            doc! {
                "index": new_block.index
            },
            doc! {
                "$set": bson::to_bson( &new_block.into_inner() ).unwrap()
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
