use bson;
use bson::oid::ObjectId;
use chrono::{DateTime, UTC};
use mongodb::{Client};
use super::*;


#[derive(Clone)]
pub struct Proposal {
    client: Client,
}

impl DBTrait for Proposal {
    fn new() -> Self {
        Proposal {
            client: Self::get_client(),
        }
    }

    fn client(&self) -> &Client {
        &self.client
    }
}

impl Proposal {

    pub fn create_proposal(&self, user_id: ObjectId, created_time: DateTime<UTC>) -> ObjectId {
        let null = bson::Bson::Null;

        let job = Self::find_one(&self.get_coll("job"), None);
        let job_id = job.get_object_id("_id").unwrap().clone();

        let doc = doc! {
            "createdTime" => created_time,
            "jobId" => job_id,
            "freelancerId" => user_id,
            "status" => "submitted",
            "statusChangeBy" => "",
            "statusReasonCode" => "",
            "statusReasonMessage" => "",
            "freelancerRead" => true,
            "clientRead" => false,
            "attachmentIds" => [ ],
            "projectIds" => [ ],
            "freelancerArchived" => false,
            "freelancerArchivedReason" => "",
            "clientArchived" => false,
            "clientArchivedReason" => "",
            "isInvite" => false,
            "isApplicant" => true,
            "estimateDuration" => "",
            "inviteLetter" => "",
            "coverLetter" => "",
            "price" => "0",
            "messaged" => false,
            "offerId" => null,
            "isDisqualified" => false
        };

        let new_id = Self::insert_one(&self.get_coll("proposal"), doc);

        new_id
    }

    pub fn create_application(&self, user_id: ObjectId, created_time: DateTime<UTC>) -> ObjectId {

        let proposal_id = self.create_proposal(user_id.clone(), created_time.clone());

        let doc = doc! {
            "createdTime" => created_time,
            "proposalId" => proposal_id,
            "userId" => user_id,
            "type" => "create",
            "comment" => "",
            "status" => "submitted"
        };

        let new_id = Self::insert_one(&self.get_coll("proposal_action"), doc);

        new_id
    }

    pub fn create_invited(&self, user_id: ObjectId, created_time: DateTime<UTC>) -> ObjectId {

        let proposal_id = self.create_proposal(user_id.clone(), created_time.clone());

        let doc = doc! {
            "createdTime" => created_time,
            "proposalId" => proposal_id,
            "userId" => user_id,
            "type" => "create",
            "comment" => "",
            "status" => "invited"
        };

        let new_id = Self::insert_one(&self.get_coll("proposal_action"), doc);

        new_id
    }

    pub fn create_matches(&self, user_id: ObjectId, created_time: DateTime<UTC>) -> ObjectId {

        let proposal_id = self.create_proposal(user_id.clone(), created_time.clone());

        let doc = doc! {
            "createdTime" => created_time,
            "proposalId" => proposal_id,
            "userId" => user_id,
            "type" => "move2active",
            "comment" => "",
            "status" => "active"
        };

        let new_id = Self::insert_one(&self.get_coll("proposal_action"), doc);

        new_id
    }

    pub fn remove_all_application(&self) {
        Self::remove_many(&self.get_coll("proposal_action"), doc!{"type" => "create", "status" => "submitted"});
    }

    pub fn remove_all_invited(&self) {
        Self::remove_many(&self.get_coll("proposal_action"), doc!{"type" => "create", "status" => "invited"});
    }

    pub fn remove_all_matches(&self) {
        Self::remove_many(&self.get_coll("proposal_action"), doc!{"type" => "move2active", "status" => "active"});
    }

    pub fn remove_all(&self) {
        Self::remove_many(&self.get_coll("proposal"), doc!{});
    }
}
