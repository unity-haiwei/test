use bson;
use bson::Bson::Null;
use bson::oid::ObjectId;
use chrono::{DateTime, UTC};
use mongodb::Client;
use super::*;


pub struct Comment {
    client: Client,
}

impl DBTrait for Comment {
    fn new() -> Self {
        Comment {
            client: Self::get_client(),
        }
    }


    fn client(&self) -> &Client {
        &self.client
    }
}

impl Comment {
    pub fn create(&self,
                  user_id: ObjectId,
                  item_type: &str,
                  item_id: ObjectId,
                  created_time: DateTime<UTC>,
                  deleted_time: Option<DateTime<UTC>>)
                  -> ObjectId {

        let deleted = match deleted_time {
            None => Null,
            Some(v) => bson::Bson::UtcDatetime(v),
        };

        let doc = doc! {
            "createdTime" => created_time,
            "updatedTime" => created_time,
            "userId" => user_id,
            "itemType" => item_type,
            "itemId" => item_id,
            "content" => "hello",
            "deletedTime" => deleted,
            "deletedBy" => Null
        };

        Self::insert_one(&self.get_coll("comment"), doc)
    }

    pub fn remove_all(&self) {
        Self::remove_many(&self.get_coll("comment"), doc!{});
    }
}
