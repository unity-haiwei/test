
use bson::Bson;
use bson::oid::{ObjectId};
use chrono::{UTC, DateTime};
use super::*;



fn create_attachment(user_id: ObjectId) -> (ObjectId, ObjectId) {
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

    let thumbnail_id = insert_one("attachment", small_picture);
    let original_id = insert_one("attachment", big_picture);

    (thumbnail_id, original_id)
}


fn create_project_content(user_id: ObjectId, project_id: ObjectId) -> ObjectId {
    let utc_now = UTC::now();

    let (thumbnail_id, original_id) = create_attachment(user_id.clone());

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

    insert_one("project_content", doc)
}


/*
    find 1 tags
    create project
    create attachment
    create project content
    update project
*/
pub fn create(user_id: ObjectId, created_time: DateTime<UTC>) {
    let null = Bson::Null;
    let user_id_clone = user_id.clone();

    let tag = find_one("tag", None);
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

    let project_id = insert_one("project", doc.clone());
    let project_id_clone = project_id.clone();

    let content_id = create_project_content(user_id.clone(), project_id.clone());
    let content_id_clone = content_id.clone();

    let project = find_one("project", Some(doc!{"_id" => project_id_clone}));

    let project_update = doc! {
        "$set" => {
            "contentIds" => [content_id],
            "coverContentId" => content_id_clone
        }
    };
    find_one_update("project", project, project_update);

    println!("Created project({}) by user({})", project_id.clone(), user_id.clone());
}
