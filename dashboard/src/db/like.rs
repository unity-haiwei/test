
use bson::oid::ObjectId;
use chrono::{DateTime, UTC};
use mongodb::Client;
use super::*;


#[derive(Clone)]
pub struct Like {
    client: Client,
}

impl DBTrait for Like {
    fn new() -> Self {
        Like {
            client: Self::get_client()
        }
    }

    fn client(&self) -> &Client {
        &self.client
    }
}

impl Like {
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
            "itemId" => item_id
        };

        Self::insert_one(&self.get_coll("like"), doc)
    }

    pub fn remove_all(&self) {
        Self::remove_many(&self.get_coll("like"), doc!{});
    }
}
