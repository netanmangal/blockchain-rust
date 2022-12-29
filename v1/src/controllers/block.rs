use mongodb::bson::{self, doc};
use mongodb::options::FindOneOptions;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};

use crate::models::block::Block;
use crate::query::block::CreateBlockInputData;

#[get("/block/get?<id>")]
pub async fn get_block_by_id(id: u32, db: &State<Database>) -> Json<Block> {
    let block: Block = db
        .collection::<Block>("block")
        .find_one(
            doc! {
                "id": id
            },
            None,
        )
        .await
        .unwrap()
        .expect("Missing block with given id.");

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
