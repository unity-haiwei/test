
pub use super::config;

use bson;
use bson::oid::ObjectId;
use bson::Document;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;

mod users;
mod projects;
mod like;
mod follow;
mod comment;
mod job;
mod team;
mod proposal;
mod message;

pub use self::users::User;
pub use self::projects::Project;
pub use self::like::Like;
pub use self::follow::Follow;
pub use self::comment::Comment;
pub use self::job::Job;
pub use self::team::Team;
pub use self::proposal::Proposal;
pub use self::message::Message;


pub trait DBTrait {

    fn new() -> Self;

    fn client(&self) -> &Client;

    fn get_coll(&self, coll_name: &str) -> Collection {
        let coll = self.client().db(config::DB_NAME).collection(coll_name);

        coll
    }

    fn get_client() -> Client {
        let client = Client::connect(config::MONGO_IP, config::MONGO_PORT)
            .ok()
            .expect("Failed to initialize mongo client.");

        client
    }

    fn find_one(coll: &Collection, doc: Option<Document>) -> Document {
        let ret = coll.find_one(doc, None)
            .ok()
            .expect(format!("Failed to find one {}.", coll.namespace).as_str());

        ret.unwrap()
    }

    fn find_all(coll: &Collection) -> Vec<Document> {
        let result = coll.find(None, None)
            .ok()
            .expect(format!("Failed to find all {}.", coll.namespace).as_str());

        result.map(|x| x.unwrap()).collect::<Vec<Document>>()
    }

    fn find_one_update(coll: &Collection, filter: Document, update: Document) -> Document {
        let ret = coll.find_one_and_update(filter, update, None)
            .ok()
            .expect(format!("Failed to find one update {}.", coll.namespace).as_str());

        ret.unwrap()
    }

    fn insert_one(coll: &Collection, doc: Document) -> ObjectId {
        let ret = coll.insert_one(doc, None)
            .ok()
            .expect(format!("Failed to Insert one doc to {}.", coll.namespace).as_str());

        bson::from_bson::<ObjectId>(ret.inserted_id.unwrap()).unwrap()
    }

    fn remove_many(coll: &Collection, doc: Document) {
        coll.delete_many(doc, None)
            .ok()
            .expect(format!("Failed to Remove doc to {}.", coll.namespace).as_str());
    }
}
