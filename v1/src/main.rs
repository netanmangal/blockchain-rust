use rocket::{self, launch};
use dotenv::dotenv;

mod db;
mod models;
mod controllers;

use db::DB;

#[launch]
async fn launch() -> _ {
    dotenv().ok();

    let db: DB = DB::connect_db().await.unwrap();
    let database_name: String = dotenv::var("DATABASE_NAME").expect("Expected env variable: DATABASE_NAME");

    rocket::build()
        .mount("/", controllers::all())
        .manage(db.client.database( &database_name ))
}