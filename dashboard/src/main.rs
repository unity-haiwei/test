
extern crate core;

#[macro_use(bson, doc)]
extern crate bson;
extern crate rustc_serialize;
extern crate mongodb;
extern crate chrono;


pub mod config;
pub mod utils;
pub mod db;

mod logic;

#[allow(unused_imports)]
use logic::{LogicTrait, Project, User};


fn main() {

    let u = User::new();
    u.run();

}
