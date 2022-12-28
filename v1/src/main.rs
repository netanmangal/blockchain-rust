use rocket::{self, launch};

mod controllers;

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/", controllers::all())
}