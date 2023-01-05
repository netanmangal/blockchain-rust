use crate::models::block::Block;
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
        .find(
            None,
            FindOptions::builder().sort(doc! {"index": 1}).build(),
        )
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();

    Json(blocks)
}
