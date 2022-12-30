use rocket::{routes, Route};

mod block;
mod index;

pub fn all() -> Vec<Route> {
    return routes![
        index::index,
        block::get_block_by_index,
        block::create_new_block,
        block::get_last_block,
        block::update_block,
        block::delete_block,
        block::get_block_count
    ];
}
