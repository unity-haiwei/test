
extern crate core;

#[macro_use(bson, doc)]
extern crate bson;
extern crate rustc_serialize;
extern crate mongodb;
extern crate chrono;


pub mod config;
pub mod db;

mod project;

fn main() {

    project::run();

}
