
use core::ops::Add;
use std::thread;

use bson;
use bson::oid::ObjectId;
use chrono::{UTC, duration};
use db;


/*
    10 projects
    40% on today(4 projects)
    5 users
    2 users created project on today
*/
fn project_view_full() {
    println!("Projects Full View ---------------------------");
    println!("10 projects, 40% on today(4 projects).");
    println!("use 5 users, 2 users created an projects on today.");

    let total_projects = 10;
    let today_projects = 6;
    let total_users = 5;
    let every_one_created = 2;

    let users = db::users::all();

    if users.len() < total_users {
        panic!("Find users < need users, Find users: {}, Need users: {}", users.len(), total_users);
    }

    let mut users_index = 0;

    for i in 0..total_projects {
        let mut created_time = UTC::now().add(duration::Duration::days(-1));
        if i >= today_projects {
            created_time = UTC::now();
        }

        let user_id = bson::from_bson::<ObjectId>(users[users_index].get("_id").unwrap().clone()).unwrap();

        db::projects::create(user_id, created_time);

        if i % every_one_created != 0 {
            users_index += 1;
        }
    }

    println!("Done Projects Full View ----------------------");
}


pub fn run() {

    let full_view = thread::spawn(|| {
        project_view_full();
    });

    full_view.join().unwrap();

}
