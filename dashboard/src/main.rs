
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongo_driver;
extern crate chrono;

use chrono::*;

pub mod config;
pub mod db;

mod project;

fn main() {

    db::project::create();

}
