
use bson::oid::ObjectId;
use chrono::{DateTime, UTC};
use mongodb::Client;
use super::*;


pub struct Follow {
    client: Client,
}

impl DBTrait for Follow {
    fn new() -> Self {
        Follow {
            client: Self::get_client(),
        }
    }


    fn client(&self) -> &Client {
        &self.client
    }
}

impl Follow {
    pub fn create(&self,
                  user_id: ObjectId,
                  item_type: &str,
                  item_id: ObjectId,
                  created_time: DateTime<UTC>)
                  -> ObjectId {

        let doc = doc! {
            "createdTime" => created_time,
            "updatedTime" => created_time,
            "userId" => user_id,
            "type" => item_type,
            "followeeId" => item_id
        };

        Self::insert_one(&self.get_coll("follow"), doc)
    }

    pub fn remove_all(&self) {
        Self::remove_many(&self.get_coll("follow"), doc!{});
    }
}
