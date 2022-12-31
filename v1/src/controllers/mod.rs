use rocket::{routes, Route};

mod block;
mod index;
mod transaction;

pub fn all() -> Vec<Route> {
    return routes![
        index::index,
        block::get_block_by_index,
        block::create_new_block,
        block::get_last_block,
        block::update_block,
        block::delete_block,
        block::get_block_count,
        transaction::get_transaction_by_hash,
        transaction::create_new_transaction,
        transaction::update_transaction,
        transaction::delete_transaction,
        transaction::get_transaction_count
    ];
}
