
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


#[derive(Clone)]
pub struct User;

impl User {
    fn generate_profile_completeness(&self) {
        println!("Profile Completeenss View ------------------ {}", UTC::now());
        println!("3 group and each group 2 users, 0 day(<20%), 3 days(<40%) and 7 days(>40%)");

        let group = 3;
        let each_group = 2;
        let need_users_count = group * each_group;
        let time_window = Duration::days(-1);

        let users = db::users::all();

        if users.len() < need_users_count {
            panic!("Find users < need users, Find users: {}, Need users: {}", users.len(), need_users_count);
        }

        let borrow_users = &users[0..need_users_count];
        let mut created_time = UTC::now().add(time_window);
        let mut group_index = 1;

        println!("Time window: {}, Begin created time: {}", time_window, created_time);

        for i in 0..borrow_users.len() {
            println!("-------------------------------------------");

            let user = &borrow_users[i];
            let user_id = bson::from_bson::<ObjectId>(user.get("_id").unwrap().clone()).unwrap();

            db::users::reset_profile_completeness(user_id.clone());

            if i != 0 && i % each_group == 0 {
                group_index += 1;

                match group_index {
                    2 => created_time = created_time.add(Duration::days(-3)),
                    3 => created_time = created_time.add(Duration::days(-4)),
                    _ => println!("Change created_time not match. group_index: {}", group_index),
                }
            }

            let user_update = match group_index {
                1 => self.update_doc(created_time, "".to_string(), "".to_string()),
                2 => self.update_doc(created_time, "Test title".to_string(), "".to_string()),
                3 => self.update_doc(created_time, "Test title".to_string(), "Test description".to_string()),
                _ => self.update_doc(created_time, "".to_string(), "".to_string()),
            };

            db::users::update_profile_completeness(user_id, user_update);

            println!("Reset user created_time: {}", created_time);
        }

        println!("End Profile Completeenss View ------------------");
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
        let self_clone = self.clone();
        thread::spawn(move || {
            self_clone.generate_profile_completeness();
        }).join().unwrap();

        let commands: Vec<&'static str> = vec!
        [
            config::COMMAND_DROP,
            config::COMMAND_SETUP,
            config::COMMAND_ZEROTH_DAY_COMPLETENESS,
            config::COMMAND_THIRD_DAY_COMPLETENESS,
            config::COMMAND_SEVENTH_DAY_COMPLETENESS
        ];
        utils::executes_commands(&commands);
    }
}
