use core::ops::Add;
use std::thread;

use bson;
use bson::oid::ObjectId;
use chrono::{UTC};
use chrono::duration::Duration;

use config;
use utils;
use db;
use super::LogicTrait;


#[derive(Clone)]
pub struct Project;

impl Project {

    pub fn new() -> Project {
        Project {}
    }

    fn full_view(&self) {
        println!("Projects Full View ------------------ {}", UTC::now());
        println!("10 projects, 40% on today(4 projects).");
        println!("use 5 users, 2 users created an projects on today.");

        let total_projects = 10;
        let today_projects = 4;
        let total_users = 5;
        let every_one_created = 2;
        let interval = Duration::minutes(4);

        let users = db::users::all();
        if users.len() < total_users {
            panic!("Find users < need users, Find users: {}, Need users: {}", users.len(), total_users);
        }

        db::projects::remove_all();

        let mut is_time_change = false;
        let mut users_index = 0;
        let mut created_time = UTC::now().add(Duration::days(-2));

        for i in 0..total_projects {
            if !is_time_change && i >= total_projects - today_projects {
                // time move left 10 minutes
                created_time = UTC::now().add(Duration::minutes(-10));
                is_time_change = true;
            }
            created_time = created_time.add(-interval);

            let user_id = bson::from_bson::<ObjectId>(users[users_index].get("_id").unwrap().clone()).unwrap();

            db::projects::create(user_id, created_time);

            if i % every_one_created != 0 {
                users_index += 1;
            }
        }

        println!("End Projects Full View ----------------------");
    }
}


impl<'a> LogicTrait for Project  {

    fn run(&self) {
        let self_clone = self.clone();
        thread::spawn(move || {
            self_clone.full_view();
        }).join().unwrap();


        let commands: Vec<&'static str> = vec!
        [
            config::COMMAND_DROP,
            config::COMMAND_SETUP,
            config::COMMAND_TOTAL_PROJECT,
            config::COMMAND_NEW_PROJECT,
            config::COMMAND_TOTAL_PROJECT_USER_SCALE,
            config::COMMAND_NEW_PROJECT_USER_SCALE
        ];
        utils::executes_commands(&commands);
    }
}
