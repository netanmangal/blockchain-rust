use dotenv::dotenv;
use rocket::{self, launch, Config};

mod controllers;
mod db;
mod models;
mod query;
mod utils;

use db::DB;

#[launch]
async fn launch() -> _ {
    // start the server using - cargo run localhost:8001

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("start the server using - cargo run localhost:8001");
    }

    dotenv().ok();

    let port: &str = args[1].split(':').collect::<Vec<&str>>()[1];

    let db: DB = DB::connect_db().await.unwrap();
    let database_name: String =
        dotenv::var("DATABASE_NAME").expect("Expected env variable: DATABASE_NAME") + "-" + port;

    let rocket_figment = Config::figment().merge(("port", port.parse::<u16>().unwrap()));

    rocket::custom(rocket_figment)
        .mount("/", controllers::all())
        .manage(db.client.database(&database_name))
}
