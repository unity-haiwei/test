
use super::{config};
use mongo_driver::client::{ClientPool, Uri};
use bson::ordered::OrderedDocument;


pub fn get_all() -> Vec<OrderedDocument> {
    let uri = Uri::new(config::MONGO_URI).unwrap();
    let pool = ClientPool::new(uri, None);
    let client = pool.pop();
    let coll = client.get_collection(config::DB_NAME, "user");

    let cursor = coll.find(&doc!{}, None).unwrap();
    let result = cursor.map(|x| x.unwrap() ).collect::<Vec<OrderedDocument>>();

    result
}
