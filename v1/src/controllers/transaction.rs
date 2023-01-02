use mongodb::bson::{self, doc};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};

use crate::models::transaction::Transaction;
use crate::query::transaction::CreateTransactionInputData;

#[get("/transaction/get?<hash>")]
pub async fn get_transaction_by_hash(hash: String, db: &State<Database>) -> Json<Transaction> {
    let transaction: Transaction = db
        .collection::<Transaction>("pending_transactions")
        .find_one(
            doc! {
                "transaction_hash": hash
            },
            None,
        )
        .await
        .unwrap()
        .expect("Missing transaction with given hash.");

    return Json(transaction);
}

#[post(
    "/transaction/create",
    format = "application/json",
    data = "<transaction_data>"
)]
pub async fn create_new_transaction(
    transaction_data: Json<CreateTransactionInputData>,
    db: &State<Database>,
) -> Json<Transaction> {
    let new_transaction: Transaction = Transaction::try_new(
        &transaction_data.creator_address,
        &transaction_data.transaction_type,
    );

    db.collection::<Transaction>("pending_transactions")
        .insert_one(&new_transaction, None)
        .await
        .ok();

    Json(new_transaction)
}

#[put(
    "/transaction/update",
    format = "application/json",
    data = "<updated_transaction>"
)]
pub async fn update_transaction(
    updated_transaction: Json<Transaction>,
    db: &State<Database>,
) -> Json<Transaction> {
    let prev_txn_hash: &String = &updated_transaction.transaction_hash;

    let new_transaction: Transaction = Transaction::try_new(
        &updated_transaction.creator_address,
        &updated_transaction.transaction_type,
    );

    db.collection::<Transaction>("pending_transactions")
        .update_one(
            doc! {
                "transaction_hash": prev_txn_hash
            },
            doc! {
                "$set": bson::to_bson( &new_transaction ).unwrap()
            },
            None,
        )
        .await
        .ok();

    Json(new_transaction)
}

#[delete("/transaction/delete?<hash>")]
pub async fn delete_transaction(hash: String, db: &State<Database>) -> String {
    db.collection::<Transaction>("pending_transactions")
        .delete_one(
            doc! {
                "transaction_hash": &hash
            },
            None,
        )
        .await
        .ok();

    format!("Transaction has been deleted successfully!!!")
}

#[get("/transaction/pending-count")]
pub async fn get_transaction_count(db: &State<Database>) -> String {
    let count: u64 = db
        .collection::<Transaction>("pending_transactions")
        .count_documents(None, None)
        .await
        .unwrap();
    count.to_string()
}
