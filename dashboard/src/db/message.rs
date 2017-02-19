use bson::oid::ObjectId;
use chrono::{DateTime, UTC};
use mongodb::{Client};
use super::*;


#[derive(Clone)]
pub struct Message {
    client: Client,
}

impl DBTrait for Message {
    fn new() -> Self {
        Message {
            client: Self::get_client(),
        }
    }

    fn client(&self) -> &Client {
        &self.client
    }
}

impl Message {

    pub fn create(&self, user_id: ObjectId, created_time: DateTime<UTC>) -> ObjectId {

        let doc = doc! {
            "type" => "normal",
            "data" => { },
            "channelId" => 0,
            "authorId" => user_id,
            "content" => "",
            "attachmentIds" => [ ],
            "nonce" => 0,
            "origContent" => "",
            "lastEditedId" => 0,
            "postTime" => created_time,
            "embeds" => [ ]
        };

        let new_id = Self::insert_one(&self.get_coll("message"), doc);

        new_id
    }

    pub fn remove_all(&self) {
        Self::remove_many(&self.get_coll("message"), doc!{});
    }
}
