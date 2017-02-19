use core::ops::Add;

use bson::Bson;
use bson::oid::{ObjectId};
use chrono::{UTC, DateTime};
use chrono::duration::Duration;
use mongodb::{Client};
use super::*;


#[derive(Clone)]
pub struct Job {
    client: Client,
}

impl DBTrait for Job {
    fn new() -> Self {
        Job {
            client: Self::get_client(),
        }
    }

    fn client(&self) -> &Client {
        &self.client
    }
}


impl Job {

    pub fn create(&self, user_id: ObjectId, job_type: &str, team_id: ObjectId, created_time: DateTime<UTC>) -> ObjectId {
        let null = Bson::Null;
        let expired_time = created_time.add(Duration::days(30));

        let place = Self::find_one(&self.get_coll("place"), None);
        let place_id = place.get_object_id("_id").unwrap().clone();
        let tag = Self::find_one(&self.get_coll("tag"), None);
        let tag_id = tag.get_object_id("_id").unwrap().clone();

        let doc = doc!{
            "createdTime" => created_time,
            "updatedTime" => created_time,
            "type" => job_type,
            "status" => "published",
            "publishTime" => created_time,
            "statusReason" => "",
            "clientId" => user_id,
            "teamId" => team_id,
            "categoryId" => "",
            "placeId" => place_id,
            "allowRemote" => false,
            "applyUrl" => "",
            "emails" => [],
            "attachmentIds" => [ ],
            "mediaIds" => [ ],
            "preferredLanguages" => [ "en" ],
            "workMode" => "",
            "title" => "Test code task",
            "visibility" => "public",
            "description" => "test code desc",
            "maxWorkHoursWeekly" => 0,
            "payType" => "hourly",
            "price" => "0",
            "headCount" => 0,
            "expLevel" => "",
            "skillIds" => [ tag_id.clone() ],
            "filled" => false,
            "workDuration" => "short",
            "workDurationUnit" => "h",
            "coverLetterRequired" => false,
            "expireTime" => expired_time,
            "unityCertified" => "preferred",
            "unityExpLevel" => "qualified",
            "currency" => "",
            "salaryRangeFrom" => "0",
            "salaryRangeTo" => "0",
            "properties" => {  },
            "deletedTime" => null,
            "deletedReason" => ""
        };

        let job_id = Self::insert_one(&self.get_coll("job"), doc.clone());

        job_id
    }

    pub fn remove_all(&self) {
        Self::remove_many(&self.get_coll("job"), doc!{});
    }

}
