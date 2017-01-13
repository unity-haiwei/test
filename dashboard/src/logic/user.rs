
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
use super::LogicTrait;

enum DataType {
    Zeroth,
    Third,
    Seventh,
}


#[derive(Clone)]
pub struct User {
    db: db::User,
    users: Vec<Document>,
}

impl User {

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

        for user in users {
            let user_id = bson::from_bson::<ObjectId>(user.get("_id").unwrap().clone()).unwrap();

            self.db.reset_profile_completeness(user_id.clone(), created_time);
        }
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


impl LogicTrait for User {

    fn new() -> User {
        let db_user = db::User::new();

        let users = db_user.all();
        let mut user_data: Vec<Document> = Vec::new();
        user_data.extend_from_slice(&users[0..2]);

        User {
            db: db_user,
            users: user_data,
        }
    }

    fn run(&self) {
        println!("Profile Completeenss View ------------------ {}", UTC::now());
        println!("Change all users created time to 10 days before and completeenss is 0%.");
        println!("Get 2 users and reset profile completeenss is 0%");
        println!("Run {}", config::COMMAND_ZEROTH_DAY_PROFILE_COMPLETENESS);
        println!("Change users created time to 3 days before and completeenss > 20%, Run {}", config::COMMAND_THIRD_DAY_PROFILE_COMPLETENESS);
        println!("Change users created time to 7 days before and completeenss > 40%, Run {}", config::COMMAND_SEVENTH_DAY_PROFILE_COMPLETENESS);

        let commands_setup: Vec<&'static str> = vec![config::COMMAND_DROP, config::COMMAND_SETUP];

        let commands_zeroth: Vec<&'static str> = vec![config::COMMAND_ZEROTH_DAY_COMPLETENESS];
        let commands_third: Vec<&'static str> = vec![config::COMMAND_THIRD_DAY_COMPLETENESS];
        let commands_seventh: Vec<&'static str> = vec![config::COMMAND_SEVENTH_DAY_COMPLETENESS];

        let commands_zeroth_profile: Vec<&'static str> = vec![config::COMMAND_ZEROTH_DAY_PROFILE_COMPLETENESS];
        let commands_third_profile: Vec<&'static str> = vec![config::COMMAND_THIRD_DAY_PROFILE_COMPLETENESS];
        let commands_seventh_profile: Vec<&'static str> = vec![config::COMMAND_SEVENTH_DAY_PROFILE_COMPLETENESS];

        let created_time = UTC::now().add(Duration::days(-8));
        let created_time_old = UTC::now().add(Duration::days(-16));


        let setup = || utils::executes_commands(&commands_setup);
        let reset_all = || self.reset_all_users();

        let generate_zeroth_old = || self.generate_data_by_day(DataType::Zeroth, created_time_old);
        let handle_zeroth_old = || utils::executes_commands(&commands_zeroth);
        let generate_third_old = || self.generate_data_by_day(DataType::Third, created_time_old);
        let handle_third_old = || utils::executes_commands(&commands_third);
        let generate_seventh_old = || self.generate_data_by_day(DataType::Seventh, created_time_old);
        let handle_seventh_old = || utils::executes_commands(&commands_seventh);

        let generate_zeroth_profile = || self.generate_data_by_day(DataType::Zeroth, created_time);
        let handle_zeroth_profile = || utils::executes_commands(&commands_zeroth_profile);
        let generate_third_profile = || self.generate_data_by_day(DataType::Third, created_time);
        let handle_third_profile = || utils::executes_commands(&commands_third_profile);
        let generate_seventh_profile = || self.generate_data_by_day(DataType::Seventh, created_time);
        let handle_seventh_profile = || utils::executes_commands(&commands_seventh_profile);


        let mut handles: Vec<&Fn()> = Vec::new();
        handles.push(&setup);
        handles.push(&reset_all);

        handles.push(&generate_zeroth_old);
        handles.push(&handle_zeroth_old);
        handles.push(&generate_third_old);
        handles.push(&handle_third_old);
        handles.push(&generate_seventh_old);
        handles.push(&handle_seventh_old);

        handles.push(&generate_zeroth_profile);
        handles.push(&handle_zeroth_profile);
        handles.push(&generate_third_profile);
        handles.push(&handle_third_profile);
        handles.push(&generate_seventh_profile);
        handles.push(&handle_seventh_profile);

        for f in &handles {
            (*f)();
        }

        println!("End Profile Completeenss View ------------------");
    }
}
