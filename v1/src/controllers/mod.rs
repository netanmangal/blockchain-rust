use rocket::{Route, routes};

pub mod index;

pub fn all() -> Vec<Route> {
    return routes![index::index];
}