use crate::models::{block::Block, transaction::Transaction};
use mongodb::{bson::DateTime, Database};

async fn create_genesis_block(db: &Database) {
    let block_count: u64 = db
        .collection::<Block>("block")
        .count_documents(None, None)
        .await
        .unwrap();

    if block_count > 0 {
        return;
    }

    let genesis_block: Block = Block::try_new(
        1,
        &String::from("0"),
        DateTime::from_millis(0),
        0,
        &Vec::<Transaction>::new(),
        &String::from("0"),
    );
    db.collection::<Block>("block")
        .insert_one(&genesis_block, None)
        .await
        .ok();
}

pub async fn init(db: &Database) {
    create_genesis_block(db).await;
    println!("Starting the server.");
}
