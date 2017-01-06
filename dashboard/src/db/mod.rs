
pub use super::config;

use bson;
use bson::oid::ObjectId;
use bson::Document;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

pub mod users;
pub mod projects;


pub fn find_one(coll_name: &str, doc: Option<Document>) -> Document {
    let client = Client::connect(config::MONGO_IP, config::MONGO_PORT)
        .ok()
        .expect("Failed to initialize mongo client.");
    let coll = client.db(config::DB_NAME).collection(coll_name);

    let ret = coll.find_one(doc, None)
        .ok()
        .expect(format!("Failed to find one {}.", coll_name).as_str());

    ret.unwrap()
}


pub fn find_all(coll_name: &str) -> Vec<Document> {
    let client = Client::connect(config::MONGO_IP, config::MONGO_PORT)
        .ok()
        .expect("Failed to initialize mongo client.");
    let coll = client.db(config::DB_NAME).collection(coll_name);

    let result = coll.find(None, None)
        .ok()
        .expect(format!("Failed to find all {}.", coll_name).as_str());

    result.map(|x| x.unwrap()).collect::<Vec<Document>>()
}


pub fn find_one_update(coll_name: &str, filter: Document, update: Document) -> Document {
    let client = Client::connect(config::MONGO_IP, config::MONGO_PORT)
        .ok()
        .expect("Failed to initialize mongo client.");
    let coll = client.db(config::DB_NAME).collection(coll_name);

    let ret = coll.find_one_and_update(filter, update, None)
        .ok()
        .expect(format!("Failed to find one update {}.", coll_name).as_str());

    ret.unwrap()
}


pub fn insert_one(coll_name: &str, doc: Document) -> ObjectId {
    let client = Client::connect(config::MONGO_IP, config::MONGO_PORT)
        .ok()
        .expect("Failed to initialize mongo client.");
    let coll = client.db(config::DB_NAME).collection(coll_name);

    let ret = coll.insert_one(doc, None)
        .ok()
        .expect(format!("Failed to Insert one doc to {}.", coll_name).as_str());

    let result_id = ret.inserted_id.unwrap();

    bson::from_bson::<ObjectId>(result_id).unwrap()
}
