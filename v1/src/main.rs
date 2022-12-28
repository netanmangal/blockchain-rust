use rocket::{self, launch};

mod models;
mod controllers;

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/", controllers::all())
}