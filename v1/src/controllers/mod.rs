use rocket::{Route, routes};

mod index;
mod block;

pub fn all() -> Vec<Route> {
    return routes![index::index, block::get_block_by_id];
}