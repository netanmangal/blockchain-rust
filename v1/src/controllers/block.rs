use mongodb::bson::{self, doc};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};

use crate::models::block::Block;

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
