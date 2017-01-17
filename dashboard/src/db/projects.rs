
use bson::Bson;
use bson::oid::{ObjectId};
use chrono::{UTC, DateTime};
use mongodb::{Client};
use super::*;

#[derive(Clone)]
pub struct Project {
    client: Client,
}

impl DBTrait for Project {

    fn new() -> Self {
        Project {
            client: Self::get_client(),
        }
    }

    fn client(&self) -> &Client {
        &self.client
    }
}

impl Project {

    pub fn create_attachment(&self, user_id: ObjectId) -> (ObjectId, ObjectId) {
        let utc_now = UTC::now();

        let uid_for_big = user_id.clone();

        let big_picture = doc!{
            "createdTime" => utc_now,
            "updatedTime" => utc_now,
            "namespace" => "unity-connect-int",
            "key" => "p/images/e0d28226-9fd3-44df-acc4-5fa704ae741a_043.jpg",
            "type" => "aws",
            "filename" => "043.jpg",
            "contentType" => "image/jpeg",
            "width" => 769,
            "height" => 900,
            "size" => 53454,
            "userId" => uid_for_big
        };

        let small_picture = doc!{
            "createdTime" => utc_now,
            "updatedTime" => utc_now,
            "namespace" => "unity-connect-int",
            "key" => "p/images/thumbnail/e0d28226-9fd3-44df-acc4-5fa704ae741a_043.jpg",
            "type" => "aws",
            "filename" => "043.jpg",
            "contentType" => "image/jpeg",
            "width" => 280,
            "height" => 210,
            "size" => 0,
            "userId" => user_id
        };

        let coll = &self.get_coll("attachment");
        let thumbnail_id = Self::insert_one(coll, small_picture);
        let original_id = Self::insert_one(coll, big_picture);

        (thumbnail_id, original_id)
    }


    pub fn create_project_content(&self, user_id: ObjectId, project_id: ObjectId) -> ObjectId {
        let utc_now = UTC::now();

        let (thumbnail_id, original_id) = self.create_attachment(user_id.clone());

        let doc = doc!{
            "createdTime" => utc_now,
            "updatedTime" => utc_now,
            "revision" => 0,
            "projectId" => project_id,
            "userId" => user_id,
            "title" => "",
            "description" => "",
            "type" => "img",
            "thumbnail" => {
                "attachmentId" => thumbnail_id,
                "width" => 280,
                "height" => 210
            },
            "originalImage" => {
                "attachmentId" => original_id,
                "width" => 769,
                "height" => 900
            },
            "skillIds" => [ ],
            "url" => "",
            "externalRef" => "",
            "externalType" => "",
            "deletedReason" => ""
        };

        Self::insert_one(&self.get_coll("project_content"), doc)
    }


    /*
        find 1 tags
        create project
        create attachment
        create project content
        update project
    */
    pub fn create(&self, user_id: ObjectId, created_time: DateTime<UTC>) -> ObjectId {
        let null = Bson::Null;
        let user_id_clone = user_id.clone();

        let tag = Self::find_one(&self.get_coll("tag"), None);
        let tag_id = tag.get_object_id("_id").unwrap();

        let doc = doc!{
            "createdTime" => created_time,
            "updatedTime" => created_time,
            "rank" => 0,
            "revision" => 0,
            "userId" => user_id_clone,
            "title" => "From auto generater",
            "license" => "",
            "description" => "desc",
            "tagIds" => [tag_id.clone()],
            "published" => true,
            "publishedTime" => created_time,
            "externalRef" => "",
            "externalType" => "",
            "contentIds" => [],
            "coverContentId" => null,
            "refUrls" => { "appleStore" => "", "googlePlay" => "", "steam" => "" },
            "deletedReason" => ""
        };

        let coll = &self.get_coll("project");
        let project_id = Self::insert_one(coll, doc.clone());
        let project_id_clone = project_id.clone();

        let content_id = self.create_project_content(user_id.clone(), project_id.clone());
        let content_id_clone = content_id.clone();

        let project = Self::find_one(coll, Some(doc!{"_id" => project_id_clone}));

        let project_update = doc! {
            "$set" => {
                "contentIds" => [content_id],
                "coverContentId" => content_id_clone
            }
        };
        Self::find_one_update(coll, project, project_update);

        project_id
    }

    pub fn remove_all(&self) {
        println!("Clear attachment.");
        Self::remove_many(&self.get_coll("attachment"), doc!{});

        println!("Clear project contents.");
        Self::remove_many(&self.get_coll("project_content"), doc!{});

        println!("Clear projects.");
        Self::remove_many(&self.get_coll("project"), doc!{});
    }
}
