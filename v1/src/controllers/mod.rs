use rocket::{routes, Route};

mod block;
mod blockchain;
mod index;
mod peer;
mod transaction;

pub fn all() -> Vec<Route> {
    return routes![
        index::index,
        block::get_block_by_index,
        block::mine_new_block,
        block::get_last_block,
        block::update_block,
        block::delete_block,
        block::get_block_count,
        transaction::get_transaction_by_hash,
        transaction::create_and_broadcast_new_transaction,
        transaction::create_new_transaction,
        transaction::update_transaction,
        transaction::delete_transaction,
        transaction::get_transaction_count,
        blockchain::get_entire_blockchain,
        peer::get_peers_list,
        peer::register_and_broadcast_node,
        peer::register_new_peer,
        peer::register_new_peers
    ];
}
