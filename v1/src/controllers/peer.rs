use mongodb::bson::doc;
use mongodb::Database;
use reqwest;
use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;
use rocket::{get, post, State};

use crate::models::peer::Peer;

async fn get_list_of_peers(db: &State<Database>) -> Vec<Peer> {
    return db
        .collection("peer")
        .find(None, None)
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();
}

async fn exists_in_db_already(db: &State<Database>, peer: &Peer) -> bool {
    let peers: Vec<Peer> = get_list_of_peers(db).await;
    let existing_peer: Vec<&Peer> = peers
        .iter()
        .filter(|p| p.address == peer.address)
        .collect::<Vec<&Peer>>();

    if existing_peer.len() != 0 {
        return true;
    } else {
        return false;
    }
}

fn get_current_node_url() -> String {
    return std::env::args().collect::<Vec<String>>()[1].clone();
}

#[get("/peers/list")]
pub async fn get_peers_list(db: &State<Database>) -> Json<Vec<Peer>> {
    Json(get_list_of_peers(db).await)
}

#[post(
    "/peers/register-and-broadcast-node",
    format = "application/json",
    data = "<new_peer>"
)]
pub async fn register_and_broadcast_node(new_peer: Json<Peer>, db: &State<Database>) -> String {
    let reqwest_client = reqwest::Client::new();
    let mut peers: Vec<Peer> = get_list_of_peers(db).await;

    // adds in own database
    if !exists_in_db_already(db, &new_peer.clone().into_inner()).await
        && get_current_node_url() != new_peer.address
    {
        db.collection::<Peer>("peer")
            .insert_one(&new_peer.clone().into_inner(), None)
            .await
            .ok();
    }

    // broadcast to other existing nodes
    for p in peers.iter() {
        reqwest_client
            .post("http://".to_owned() + &p.address.clone() + "/peers/register")
            .json::<Peer>(&new_peer.clone().into_inner())
            .send()
            .await
            .ok();
    }

    // sends existing peers to the new node
    peers.push(Peer {
        address: get_current_node_url(),
    });

    println!("{:?}", peers);

    reqwest_client
        .post("http://".to_owned() + &new_peer.address.clone() + "/peers/register-bulk")
        .json::<Vec<Peer>>(&peers)
        .send()
        .await
        .ok();

    format!("New peer has been successfully registered and broadcasted!!!")
}

#[post("/peers/register", format = "application/json", data = "<new_peer>")]
pub async fn register_new_peer(new_peer: Json<Peer>, db: &State<Database>) -> String {
    if !exists_in_db_already(db, &new_peer.clone().into_inner()).await
        && get_current_node_url() != new_peer.address
    {
        db.collection::<Peer>("peer")
            .insert_one(&new_peer.clone().into_inner(), None)
            .await
            .ok();
    }

    format!("New peer has been successfully registered!!!")
}

#[post(
    "/peers/register-bulk",
    format = "application/json",
    data = "<new_peers>"
)]
pub async fn register_new_peers(new_peers: Json<Vec<Peer>>, db: &State<Database>) -> String {
    for p in new_peers.into_inner().iter() {
        if !exists_in_db_already(db, &p.clone()).await
            && get_current_node_url() != p.address
        {
            db.collection::<Peer>("peer")
                .insert_one(p, None)
                .await
                .ok();
        }
    }

    format!("New peers has been successfully registered!!!")
}
