use bson;
use bson::Document;
use bson::oid::ObjectId;
use chrono::{DateTime, UTC};
use mongodb::{Client};
use super::*;


#[derive(Clone)]
pub struct Team {
    client: Client,
}

impl DBTrait for Team {
    fn new() -> Self {
        Team {
            client: Self::get_client(),
        }
    }

    fn client(&self) -> &Client {
        &self.client
    }
}

impl Team {

    pub fn create(&self, user_id: ObjectId, created_time: DateTime<UTC>, default: bool) -> ObjectId {
        let team_name = format!("Test name {}", user_id.clone());
        let team_slug = format!("testslug_{}", user_id.clone());
        let null = bson::Bson::Null;

        let place = Self::find_one(&self.get_coll("place"), None);
        let place_id = place.get_object_id("_id").unwrap().clone();

        let doc = doc! {
            "createdTime" => created_time,
            "userId" => user_id,
            "default" => default,
            "name" => team_name,
            "slug" => team_slug,
            "description" => "",
            "isPartner" => false,
            "placeId" => place_id,
            "websiteUrl" => "",
            "referenceUrls" => { },
            "coverImage" => "",
            "thumbnail" => "",
            "deletedTime" => null
        };

        let team_id = Self::insert_one(&self.get_coll("team"), doc);

        team_id
    }

    pub fn get_default_team(&self) -> Document {
        Self::find_one(&self.get_coll("team"), Some(doc!{ "default" => true}))
    }

    pub fn get_not_default_team(&self) -> Document {
        Self::find_one(&self.get_coll("team"), Some(doc!{ "default" => false}))
    }

    pub fn remove_all(&self) {
        Self::remove_many(&self.get_coll("team"), doc!{"default" => false});
    }
}
