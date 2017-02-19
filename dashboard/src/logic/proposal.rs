use core::ops::Add;

use bson::Document;
use chrono::{DateTime, UTC};
use chrono::duration::Duration;

use config;
use utils;
use db::{self, DBTrait};
use Runtime;
use super::LogicTrait;


pub struct Proposal<'a> {
    runtime: &'a Runtime,
    db: db::Proposal,
    users: Vec<Document>,
}

impl<'a> Proposal<'a> {
    fn generate_application(&self, amount: u32, user_index: &mut usize, created_time: DateTime<UTC>) {
        for _ in 0..amount {
            let user = &self.users[*user_index];
            let user_id = utils::get_obj_id(&user);

            let app_id = self.db.create_application(user_id.clone(), created_time);

            println!("Created Application({}), User({})", app_id, user_id);

            *user_index += 1;
        }
    }

    fn generate_invited(&self, amount: u32, user_index: &mut usize, created_time: DateTime<UTC>) {
        for _ in 0..amount {
            let user = &self.users[*user_index];
            let user_id = utils::get_obj_id(&user);

            let id = self.db.create_invited(user_id.clone(), created_time);

            println!("Created Invited({}), User({})", id, user_id);

            *user_index += 1;
        }
    }

    fn generate_matches(&self, amount: u32, user_index: &mut usize, created_time: DateTime<UTC>) {
        for _ in 0..amount {
            let user = &self.users[*user_index];
            let user_id = utils::get_obj_id(&user);

            let id = self.db.create_matches(user_id.clone(), created_time);

            println!("Created Match({}), User({})", id, user_id);

            *user_index += 1;
        }
    }

    fn remove_all(&self) {
        self.db.remove_all_application();
        self.db.remove_all_invited();
        self.db.remove_all_matches();
    }
}

impl<'a> LogicTrait<'a> for Proposal<'a> {
    fn new(r: &'a Runtime) -> Proposal<'a> {
        Proposal {
            runtime: r,
            db: db::Proposal::new(),
            users: db::User::new().all(),
        }
    }

    fn run(&self) {
        println!("Application Full View -------------------------- {}", UTC::now());
        println!("Create 2 copies of the data.");

        let commands_setup = vec![config::COMMAND_DROP, config::COMMAND_SETUP];
        let commands_sync_application = vec![config::COMMAND_TOTAL_APPLICATION,
                                     config::COMMAND_NEW_APPLICATION,
                                     config::COMMAND_TOTAL_APPLICATION_USER_PERCENT,
                                     config::COMMAND_DAILY_APPLICATION_USER_PERCENT];
        let commands_sync_invited = vec![config::COMMAND_TOTAL_INVITED,
                                         config::COMMAND_NEW_INVITED,
                                         config::COMMAND_TOTAL_INVITED_USER_PERCENT,
                                         config::COMMAND_DAILY_INVITED_USER_PERCENT];
        let commands_sync_match = vec![config::COMMAND_TOTAL_MATCH,
                                       config::COMMAND_NEW_MATCH,
                                       config::COMMAND_TOTAL_MATCH_USER_PERCENT,
                                       config::COMMAND_DAILY_MATCH_USER_PERCENT];

        println!("Clear all data --------------------------------------");
        self.remove_all();
        utils::executes_commands(self.runtime.script_path, &commands_setup, None, None);

        let begin_time = UTC::now().add(Duration::days(-5));
        let mut user_index = 0;
        let mut created_time = begin_time.clone();
        println!("Move time window to {}", created_time);

        self.generate_application(1, &mut user_index, created_time);
        self.generate_invited(1, &mut user_index, created_time);
        self.generate_matches(1, &mut user_index, created_time);

        created_time = created_time.add(Duration::days(1));
        println!("Move time window to {}", created_time);

        self.generate_application(2, &mut user_index, created_time);
        self.generate_invited(2, &mut user_index, created_time);
        self.generate_matches(2, &mut user_index, created_time);

        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_application,
                                 Some(begin_time),
                                 Some(created_time.add(Duration::days(1))));
        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_invited,
                                 Some(begin_time),
                                 Some(created_time.add(Duration::days(1))));
        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_match,
                                 Some(begin_time),
                                 Some(created_time.add(Duration::days(1))));

        created_time = created_time.add(Duration::days(2));
        println!("Move time window to {}", created_time);

        self.generate_application(2, &mut user_index, created_time);
        self.generate_invited(2, &mut user_index, created_time);
        self.generate_matches(2, &mut user_index, created_time);

        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_application,
                                 Some(created_time.add(Duration::days(-1))),
                                 Some(created_time.add(Duration::days(1))));
        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_invited,
                                 Some(created_time.add(Duration::days(-1))),
                                 Some(created_time.add(Duration::days(1))));
        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_match,
                                 Some(created_time.add(Duration::days(-1))),
                                 Some(created_time.add(Duration::days(1))));
    }
}
