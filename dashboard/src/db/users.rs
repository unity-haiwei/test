
use bson::Document;
use bson::oid::{ObjectId};
use chrono::{UTC, DateTime};
use mongodb::{Client};
use super::*;

#[derive(Clone)]
pub struct User {
    client: Client,
}

impl DBTrait for User {

    fn new() -> Self {
        User {
            client: Self::get_client(),
        }
    }

    fn client(&self) -> &Client {
        &self.client
    }
}


impl User {

    pub fn all(&self) -> Vec<Document> {
        Self::find_all(&self.get_coll("user"))
    }

    pub fn remove_work_experience(&self, user_id: ObjectId) {
        Self::remove_many(&self.get_coll("work_experience"), doc!{"userId" => user_id});
    }

    pub fn remove_education_experience(&self, user_id: ObjectId) {
        Self::remove_many(&self.get_coll("education_experience"), doc!{"userId" => user_id});
    }

    pub fn remove_projects(&self, user_id: ObjectId) {
        Self::remove_many(&self.get_coll("project"), doc!{"userId" => user_id});
    }

    pub fn reset_profile_completeness(&self, user_id: ObjectId, created_time: DateTime<UTC>) {
        let user_coll = &self.get_coll("user");

        let user_id_clone = user_id.clone();
        let user = Self::find_one(user_coll, Some(doc!{"_id" => user_id_clone}));

        let user_update = doc! {
            "$set" => {
                "avatar" => "",
                "title" => "",
                "description" => "",
                "placeId" => [],
                "skillIds" => [],
                "createdTime" => created_time
            }
        };
        Self::find_one_update(user_coll, user, user_update);

        self.remove_work_experience(user_id.clone());
        self.remove_education_experience(user_id.clone());
        self.remove_projects(user_id.clone());
    }

    pub fn update_profile_completeness(&self, user_id: ObjectId, doc: Document) {
        let user_coll = &self.get_coll("user");

        let user = Self::find_one(user_coll, Some(doc!{"_id" => user_id}));
        Self::find_one_update(user_coll, user, doc);
    }
}
