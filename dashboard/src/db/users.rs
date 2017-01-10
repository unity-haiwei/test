
use bson::Document;
use bson::oid::{ObjectId};
use super::*;


pub fn all() -> Vec<Document> {
    find_all("user")
}


pub fn remove_work_experience(user_id: ObjectId) {
    remove_many("work_experience", doc!{"userId" => user_id});
}


pub fn remove_education_experience(user_id: ObjectId) {
    remove_many("education_experience", doc!{"userId" => user_id});
}


pub fn reset_profile_completeness(user_id: ObjectId) {
    println!("Reset profile completeness. UserId: {}", user_id);

    let user_id_clone = user_id.clone();
    let user = find_one("user", Some(doc!{"_id" => user_id_clone}));

    let user_update = doc! {
        "$set" => {
            "avatar" => "",
            "title" => "",
            "description" => "",
            "placeId" => [],
            "skillIds" => []
        }
    };
    find_one_update("user", user, user_update);

    remove_work_experience(user_id.clone());
    remove_education_experience(user_id.clone());
}


pub fn update_profile_completeness(user_id: ObjectId, doc: Document) {
    println!("Update profile completeness. UserId: {}", user_id.clone());
    
    let user = find_one("user", Some(doc!{"_id" => user_id}));

    find_one_update("user", user, doc);
}
