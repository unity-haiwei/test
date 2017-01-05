
use super::{config};
use mongo_driver::client::{ClientPool, Uri};
use bson::ordered::OrderedDocument;
use bson::oid::{ObjectId};
use chrono::{UTC};


pub fn get_all() -> Vec<OrderedDocument> {
    let uri = Uri::new(config::MONGO_URI).unwrap();
    let pool = ClientPool::new(uri, None);
    let client = pool.pop();
    let coll = client.get_collection(config::DB_NAME, "user");

    let cursor = coll.find(&doc!{}, None).unwrap();
    let result = cursor.map(|x| x.unwrap() ).collect::<Vec<OrderedDocument>>();

    result
}


pub fn create(user_id: &str) {

    let utc_now = UTC::now();
    let uid = ObjectId::with_string(user_id).unwrap();
    let tag_id = ObjectId::with_string("586b5fd268514d1dc3cfd487").unwrap();
    let content_id = ObjectId::with_string("586dda9f68514d1681112778").unwrap();
    let cover_content_id = ObjectId::with_string("586dda9f68514d1681112778").unwrap();

    let doc = doc!{
        "createdTime" => utc_now,
        "updatedTime" => utc_now,
        "rank" => 0,
        "revision" => 0,
        "userId" => uid,
        "title" => "from test code",
        "license" => "",
        "description" => "desc",
        "tagIds" => [ tag_id],
        "published" => true,
        "publishedTime" => utc_now,
        "externalRef" => "",
        "externalType" => "",
        "contentIds" => [ content_id ],
        "coverContentId" => cover_content_id,
        "refUrls" => { "appleStore" => "", "googlePlay" => "", "steam" => "" },
        "deletedReason" => ""
    };

    let uri = Uri::new(config::MONGO_URI).unwrap();
    let pool = ClientPool::new(uri, None);
    let client = pool.pop();
    let coll = client.get_collection(config::DB_NAME, "project");

    coll.insert(&doc, None).unwrap();

    println!("Create Project");
}
