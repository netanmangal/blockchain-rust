use rocket::{self, get, launch, routes};

#[get("/")]
pub fn index() -> String {
    return format!("Welcome to homepage!");
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index])
}
