use core::ops::Add;

use bson::Document;
use bson::oid::ObjectId;
use chrono::{UTC};
use chrono::duration::Duration;

use config;
use utils;
use db;
use db::DBTrait;
use super::LogicTrait;


pub struct Follow {
    db: db::Follow,
    db_project: db::Project,
    users: Vec<Document>,
}

impl Follow {
    fn generate_by_day(&self, day: i64, project_id: ObjectId) {
        let created_time = UTC::now().add(Duration::days(-1)).add(Duration::days(-day));

        println!("{} day -------------------------------------------------", day);

        for i in 1..day + 1 {
            created_time.add(Duration::minutes(-i * 10));
            let user_id = utils::get_obj_id(&self.users[(i * day) as usize]);

            self.db.create(user_id.clone(),
                           config::ITEM_TYPE_PROJECT,
                           project_id.clone(),
                           created_time);

            println!("Created Follow. UserId: {:?}, Created time: {}", user_id, created_time);
        }
    }

    fn remove_all(&self) {
        println!("------------------------------------------------------");
        println!("Remove all follow data");
        self.db.remove_all();
    }
}

impl LogicTrait for Follow {
    fn new() -> Self {
        let db_user = db::User::new();

        Follow {
            db: db::Follow::new(),
            db_project: db::Project::new(),
            users: db_user.all(),
        }
    }

    fn run(&self) {

        println!("Follow Full View -------------------------- {}", UTC::now());
        println!("Create 2 days data");
        println!("First Day: follow a project use 1 user.");
        println!("Second Day: follow a project use 2 users.");

        let commands_setup = vec![config::COMMAND_DROP, config::COMMAND_SETUP];
        let commands_sync = vec![config::COMMAND_TOTAL_FOLLOW,
                                 config::COMMAND_NEW_FOLLOW,
                                 config::COMMAND_TOTAL_FOLLOW_USER_SCALE,
                                 config::COMMAND_NEW_FOLLOW_USER_SCALE];


        let user_id = utils::get_obj_id(&(self.users[0]));
        let project_id = self.db_project.create(user_id, UTC::now().add(Duration::days(-5)));

        let mut handles: Vec<Box<Fn()>> = Vec::new();

        handles.push(Box::new(|| utils::executes_commands(&commands_setup, None, None)));
        handles.push(Box::new(|| self.remove_all()));
        handles.push(Box::new(|| self.generate_by_day(2, project_id.clone())));
        handles.push(Box::new(|| self.generate_by_day(1, project_id.clone())));
        handles.push(Box::new(|| utils::executes_commands(&commands_sync, None, None)));

        for h in handles {
            (*h)();
        }

        println!("End Follow Full View -------------------------- ");
    }
}
