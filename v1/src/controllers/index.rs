use rocket::get;

#[get("/")]
pub fn index() -> String {
    return format!("Welcome to homepage!");
}