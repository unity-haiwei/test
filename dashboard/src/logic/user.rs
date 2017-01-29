
use core::ops::Add;

use bson;
use bson::Document;
use bson::oid::ObjectId;
use chrono::{UTC, DateTime};
use chrono::duration::Duration;

use config;
use utils;
use db;
use db::DBTrait;
use Runtime;
use super::LogicTrait;

#[derive(Clone)]
enum DataType {
    Zeroth,
    Third,
    Seventh,
}


#[derive(Clone)]
pub struct User<'a> {
    runtime: &'a Runtime,
    db: db::User,
    users: Vec<Document>,
}

impl<'a> User<'a> {

    fn generate_data_by_day(&self, day: DataType, created_time: DateTime<UTC>) {
        println!("--------------------------------------------");
        println!("Generate data for {}", match day {
            DataType::Zeroth => "0 day",
            DataType::Third => "3 days",
            DataType::Seventh => "7 days",
        });

        let user_update = match day {
            DataType::Zeroth => {
                let update_doc = self.update_doc(created_time, "".to_string(), "".to_string());

                println!("Created time: {}", created_time);

                update_doc
            },
            DataType::Third => {
                let update_doc = self.update_doc(created_time, "Test title".to_string(), "".to_string());

                println!("Created time: {}", created_time);

                update_doc
            },
            DataType::Seventh => {
                let update_doc = self.update_doc(created_time, "Test title".to_string(), "Test description".to_string());

                println!("Created time: {}", created_time);

                update_doc
            },
        };

        for user in self.clone().users {
            let user_id = bson::from_bson::<ObjectId>(user.get("_id").unwrap().clone()).unwrap();

            self.db.reset_profile_completeness(user_id.clone(), created_time);
            println!("Reset profile completeness. UserId: {}", user_id);

            self.db.update_profile_completeness(user_id.clone(), user_update.clone());
            println!("Update profile completeness. UserId: {}", user_id.clone());
        }
    }

    fn reset_all_users(&self) {
        println!("--------------------------------------------");
        println!("Reset all users profile completeness is 0%.");
        let users = self.db.all();
        let time_window = Duration::days(-10);
        let created_time = UTC::now().add(time_window);

        let _ = users.iter()
                     .map(|ref x| {
                         let user_id = utils::get_obj_id(x);
                         self.db.reset_profile_completeness(user_id.clone(), created_time);
                     });
    }

    fn update_doc(&self, created_time: DateTime<UTC>, title: String, description: String) -> Document {
        doc! {
            "$set" => {
                "avatar" => "https://d2s1aoejs9crsd.cloudfront.net/assets/styles/i/banner.375433d6f47b6f97d2ca55e148f17104.jpg",
                "title" => title,
                "description" => description,
                "placeId" => [],
                "skillIds" => [],
                "createdTime" => created_time
            }
        }
    }
}


impl<'a> LogicTrait<'a> for User<'a> {

    fn new(r: &'a Runtime) -> User<'a> {
        let db_user = db::User::new();

        let users = db_user.all();
        let mut user_data: Vec<Document> = Vec::new();
        user_data.extend_from_slice(&users[0..2]);

        User {
            runtime: r,
            db: db_user,
            users: user_data,
        }
    }

    fn run(&self) {
        println!("Profile Completeenss View ------------------ {}", UTC::now());
        println!("Create 2 copies of data samples.");
        println!("Change all users created time to 8 days before and completeenss is 0%.");
        println!("Run {}", config::COMMAND_ZEROTH_DAY_COMPLETENESS);
        println!("Change users created time to 3 days before and completeenss > 20%, Run {}", config::COMMAND_THIRD_DAY_COMPLETENESS);
        println!("Change users created time to 7 days before and completeenss > 40%, Run {}", config::COMMAND_SEVENTH_DAY_COMPLETENESS);

        let commands_setup: Vec<&'static str> = vec![config::COMMAND_DROP, config::COMMAND_SETUP];
        let commands_zeroth: Vec<&'static str> = vec![config::COMMAND_ZEROTH_DAY_COMPLETENESS];
        let commands_third: Vec<&'static str> = vec![config::COMMAND_THIRD_DAY_COMPLETENESS];
        let commands_seventh: Vec<&'static str> = vec![config::COMMAND_SEVENTH_DAY_COMPLETENESS];

        let setup = || utils::executes_commands(self.runtime.script_path, &commands_setup, None, None);
        let reset_all = || self.reset_all_users();

        let days = vec![DataType::Zeroth, DataType::Third, DataType::Seventh];

        let mut handles: Vec<Box<Fn()>> = Vec::new();
        handles.push(Box::new(setup));
        handles.push(Box::new(reset_all));

        let created_time = UTC::now().add(Duration::days(-8));

        for day in &days {
            let generate_box = Box::new(move || self.generate_data_by_day((*day).clone(), created_time.clone()));
            handles.push(generate_box);

            let command_arr = match *day {
                DataType::Zeroth => commands_zeroth.clone(),
                DataType::Third => commands_third.clone(),
                DataType::Seventh => commands_seventh.clone(),
            };
            handles.push(Box::new(move || utils::executes_commands(self.runtime.script_path, &command_arr, None, None)));
        }

        for f in &handles {
            (*f)();
        }

        println!("End Profile Completeenss View ------------------");
    }
}
