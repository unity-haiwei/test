
use bson::Document;
use super::*;

pub fn all() -> Vec<Document> {
    find_all("user")
}
