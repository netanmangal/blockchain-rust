use dotenv::dotenv;
use rocket::{self, launch};

mod controllers;
mod db;
mod models;
mod query;

use db::DB;

#[launch]
async fn launch() -> _ {
    dotenv().ok();

    let db: DB = DB::connect_db().await.unwrap();
    let database_name: String =
        dotenv::var("DATABASE_NAME").expect("Expected env variable: DATABASE_NAME");

    rocket::build()
        .mount("/", controllers::all())
        .manage(db.client.database(&database_name))
}
