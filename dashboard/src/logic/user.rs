
use core::ops::Add;
use std::thread;

use bson;
use bson::Document;
use bson::oid::ObjectId;
use chrono::{UTC, DateTime};
use chrono::duration::Duration;

use config;
use utils;
use db;
use super::LogicTrait;

enum DataType {
    Zeroth,
    Third,
    Seventh,
}


#[derive(Clone)]
pub struct User {
    users: Vec<Document>,
}

impl User {

    pub fn new() -> User {
        let users = db::users::all();
        let mut user_data: Vec<Document> = Vec::new();
        user_data.extend_from_slice(&users[0..2]);

        User {
            users: user_data,
        }
    }

    fn generate_data_by_day(&self, day: DataType) {
        println!("--------------------------------------------");
        println!("Generate data for {}", match day {
            DataType::Zeroth => "0 day",
            DataType::Third => "3 days",
            DataType::Seventh => "7 days",
        });

        let (created_time, user_update) = match day {
            DataType::Zeroth => {
                let time_window = Duration::days(-8);
                let created_time = UTC::now().add(time_window);
                let update_doc = self.update_doc(created_time, "".to_string(), "".to_string());

                println!("Time window: {}, Begin created time: {}", time_window, created_time);

                (created_time, update_doc)
            },
            DataType::Third => {
                let time_window = Duration::days(-8);
                let created_time = UTC::now().add(time_window);
                let update_doc = self.update_doc(created_time, "Test title".to_string(), "".to_string());

                println!("Time window: {}, Begin created time: {}", time_window, created_time);

                (created_time, update_doc)
            },
            DataType::Seventh => {
                let time_window = Duration::days(-8);
                let created_time = UTC::now().add(time_window);
                let update_doc = self.update_doc(created_time, "Test title".to_string(), "Test description".to_string());

                println!("Time window: {}, Begin created time: {}", time_window, created_time);

                (created_time, update_doc)
            },
        };

        for user in self.clone().users {
            let user_id = bson::from_bson::<ObjectId>(user.get("_id").unwrap().clone()).unwrap();

            db::users::reset_profile_completeness(user_id.clone(), created_time);
            println!("Reset profile completeness. UserId: {}", user_id);

            db::users::update_profile_completeness(user_id.clone(), user_update.clone());
            println!("Update profile completeness. UserId: {}", user_id.clone());
        }
    }

    fn reset_all_users(&self) {
        let users = db::users::all();
        let time_window = Duration::days(-10);
        let created_time = UTC::now().add(time_window);

        for user in users {
            let user_id = bson::from_bson::<ObjectId>(user.get("_id").unwrap().clone()).unwrap();

            db::users::reset_profile_completeness(user_id.clone(), created_time);
        }
    }

    fn update_doc(&self, created_time: DateTime<UTC>, title: String, description: String) -> Document {
        doc! {
            "$set" => {
                "avatar" => "https://odesk-prod-portraits.s3.amazonaws.com/Users:aroqman:PortraitUrl_100?AWSAccessKeyId=1XVAX3FNQZAFC9GJCFR2&Expires=2147483647&Signature=w77jBvIotyp583acchGdIJPrPWw%3D",
                "title" => title,
                "description" => description,
                "placeId" => [],
                "skillIds" => [],
                "createdTime" => created_time
            }
        }
    }
}


impl<'a> LogicTrait for User  {

    fn run(&self) {
        println!("Profile Completeenss View ------------------ {}", UTC::now());
        println!("Change all users created time to 10 days before and completeenss is 0%.");
        println!("Get 2 users and reset profile completeenss is 0%");
        println!("Run {}", config::COMMAND_ZEROTH_DAY_COMPLETENESS);
        println!("Change users created time to 3 days before and completeenss > 20%, Run {}", config::COMMAND_THIRD_DAY_COMPLETENESS);
        println!("Change users created time to 7 days before and completeenss > 40%, Run {}", config::COMMAND_SEVENTH_DAY_COMPLETENESS);

        let commands_setup: Vec<&'static str> = vec!
        [
            config::COMMAND_DROP,
            config::COMMAND_SETUP
        ];
        let commands_zeroth: Vec<&'static str> = vec!
        [
            config::COMMAND_ZEROTH_DAY_COMPLETENESS
        ];
        let commands_third: Vec<&'static str> = vec!
        [
            config::COMMAND_THIRD_DAY_COMPLETENESS
        ];
        let commands_seventh: Vec<&'static str> = vec!
        [
            config::COMMAND_SEVENTH_DAY_COMPLETENESS
        ];

        let setup = || utils::executes_commands(&commands_setup);
        let reset_all = || self.reset_all_users();

        let generate_zeroth = || self.generate_data_by_day(DataType::Zeroth);
        let handle_zeroth = || utils::executes_commands(&commands_zeroth);

        let generate_third = || self.generate_data_by_day(DataType::Third);
        let handle_third = || utils::executes_commands(&commands_third);

        let generate_seventh = || self.generate_data_by_day(DataType::Seventh);
        let handle_seventh = || utils::executes_commands(&commands_seventh);


        let mut handles: Vec<&Fn()> = Vec::new();
        handles.push(&setup);
        handles.push(&reset_all);
        handles.push(&generate_zeroth);
        handles.push(&handle_zeroth);
        handles.push(&generate_third);
        handles.push(&handle_third);
        handles.push(&generate_seventh);
        handles.push(&handle_seventh);

        for f in &handles {
            (*f)();
        }

        println!("End Profile Completeenss View ------------------");
    }
}
