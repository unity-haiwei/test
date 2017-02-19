use core::ops::Add;

use bson::Document;
use chrono::{DateTime, UTC};
use chrono::duration::Duration;

use config;
use utils;
use db::{self, DBTrait};
use Runtime;
use super::LogicTrait;


pub struct Message<'a> {
    runtime: &'a Runtime,
    db: db::Message,
    users: Vec<Document>,
}

impl<'a> Message<'a> {
    fn generate(&self, amount: u32, user_index: &mut usize, created_time: DateTime<UTC>) {
        for _ in 0..amount {
            let user = &self.users[*user_index];
            let user_id = utils::get_obj_id(&user);

            let id = self.db.create(user_id.clone(), created_time);

            println!("Created Message({}), User({})", id, user_id);

            *user_index += 1;
        }
    }

    fn remove_all(&self) {
        self.db.remove_all();
    }
}

impl<'a> LogicTrait<'a> for Message<'a> {
    fn new(r: &'a Runtime) -> Message<'a> {
        Message {
            runtime: r,
            db: db::Message::new(),
            users: db::User::new().all(),
        }
    }

    fn run(&self) {
        println!("Message Full View -------------------------- {}", UTC::now());
        println!("Create 2 copies of the data.");

        let commands_setup = vec![config::COMMAND_DROP, config::COMMAND_SETUP];
        let commands_sync_message = vec![config::COMMAND_TOTAL_MESSAGE,
                                     config::COMMAND_NEW_MESSAGE,
                                     config::COMMAND_TOTAL_MESSAGE_USER_PERCENT,
                                     config::COMMAND_DAILY_MESSAGE_USER_PERCENT];

        println!("Clear all data --------------------------------------");
        self.remove_all();
        utils::executes_commands(self.runtime.script_path, &commands_setup, None, None);

        let begin_time = UTC::now().add(Duration::days(-5));
        let mut user_index = 0;
        let mut created_time = begin_time.clone();
        println!("Move time window to {}", created_time);

        self.generate(1, &mut user_index, created_time);

        created_time = created_time.add(Duration::days(1));
        println!("Move time window to {}", created_time);

        self.generate(2, &mut user_index, created_time);

        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_message,
                                 Some(begin_time),
                                 Some(created_time.add(Duration::days(1))));

        created_time = created_time.add(Duration::days(2));
        println!("Move time window to {}", created_time);

        self.generate(2, &mut user_index, created_time);

        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_message,
                                 Some(created_time.add(Duration::days(-1))),
                                 Some(created_time.add(Duration::days(1))));
    }
}
