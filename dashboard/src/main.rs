
extern crate core;

#[macro_use(bson, doc)]
extern crate bson;
extern crate rustc_serialize;
extern crate mongodb;
extern crate chrono;
#[macro_use]
extern crate clap;


pub mod config;
pub mod utils;
pub mod db;

mod runtime;
mod logic;

pub use runtime::Runtime;

use clap::{App, Arg};
#[allow(unused_imports)]
use logic::*;


fn main() {

    let runtime = &Runtime {
        script_path: "/Volumes/work/unity/genesis-backend/dashboard/scripts/marketplace",
    };

    let matches = App::new("DataGerenater")
                        .version("1.0")
                        .arg(Arg::with_name("type")
                                 .help("Choice data type generater")
                                 .possible_values(&["user", "project", "like", "follow", "comment"])
                                 .required(true)
                                 .takes_value(true))
                        .arg(Arg::with_name("reset")
                                 .short("r")
                                 .long("reset")
                                 .help("Reset influxdb, run drop and setup command.")
                                 .multiple(true)
                                 .requires("type"))
                        .get_matches();

    if matches.is_present("reset") {
        println!("reset is turned on");
    }

    match matches.value_of("type").unwrap() {
        "user" => User::new(runtime).run(),
        "project" => Project::new(runtime).run(),
        "like" => Like::new(runtime).run(),
        "follow" => Follow::new(runtime).run(),
        "comment" => Comment::new(runtime).run(),
        _ => User::new(runtime).run(),
    }

}
