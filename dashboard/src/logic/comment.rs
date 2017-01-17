use core::ops::Add;

use bson::Document;
use bson::oid::ObjectId;
use chrono::{UTC, TimeZone};
use chrono::duration::Duration;

use config;
use utils;
use db;
use db::DBTrait;
use super::LogicTrait;


pub struct Comment {
    db: db::Comment,
    db_project: db::Project,
    users: Vec<Document>,
}

impl Comment {
    fn generate_by_day(&self, day: i64, project_id: ObjectId) {
        let created_time = UTC::now().add(Duration::days(-1)).add(Duration::days(-day));

        println!("{} day -------------------------------------------------", day);

        for i in 1..day + 1 {
            created_time.add(Duration::minutes(-i * 10));
            let user_id = utils::get_obj_id(&self.users[(i * day) as usize]);

            self.db.create(user_id.clone(),
                           config::ITEM_TYPE_PROJECT,
                           project_id.clone(),
                           created_time,
                           None);

            println!("Created Comment. UserId: {:?}, Created time: {}", user_id, created_time);
        }
    }

    fn generate_deleted_record(&self, user_id: ObjectId, project_id: ObjectId) {
        let created_time = UTC::now().add(Duration::days(-1));

        println!("Delted Record -------------------------------------------------");

        self.db.create(user_id.clone(),
                       config::ITEM_TYPE_PROJECT,
                       project_id.clone(),
                       created_time,
                       Some(created_time));

        println!("Created Comment. UserId: {:?}, Created time: {}", user_id, created_time);
    }

    fn remove_all(&self) {
        println!("------------------------------------------------------");
        println!("Remove all comment data");
        self.db.remove_all();
    }
}

impl LogicTrait for Comment {
    fn new() -> Self {
        let db_user = db::User::new();

        Comment {
            db: db::Comment::new(),
            db_project: db::Project::new(),
            users: db_user.all(),
        }
    }

    fn run(&self) {

        println!("Comment Full View -------------------------- {}", UTC::now());
        println!("Create 2 days data and deleted record.");
        println!("First Day: Comment to a project use 1 user.");
        println!("Second Day: Comment to a project use 2 users.");

        let commands_setup = vec![config::COMMAND_DROP, config::COMMAND_SETUP];
        let commands_sync = vec![config::COMMAND_TOTAL_COMMENT,
                                 config::COMMAND_NEW_COMMENT,
                                 config::COMMAND_TOTAL_COMMENT_USER_SCALE,
                                 config::COMMAND_NEW_COMMENT_USER_SCALE];

        let user_id = utils::get_obj_id(&(self.users[0]));
        let project_id = self.db_project.create(user_id, UTC::now().add(Duration::days(-5)));
        let start_time = UTC.ymd(2017, 1, 1).and_hms_opt(0, 0, 0);
        let end_time = UTC.ymd(2017, 1, 17).and_hms_opt(0, 0, 0);

        let mut handles: Vec<Box<Fn()>> = Vec::new();

        handles.push(Box::new(|| utils::executes_commands(&commands_setup, start_time, end_time)));
        handles.push(Box::new(|| self.remove_all()));
        handles.push(Box::new(|| self.generate_by_day(2, project_id.clone())));
        handles.push(Box::new(|| self.generate_by_day(1, project_id.clone())));
        handles.push(Box::new(|| self.generate_deleted_record(utils::get_obj_id(&(self.users[1])), project_id.clone())));
        handles.push(Box::new(|| utils::executes_commands(&commands_sync, start_time, end_time)));

        for h in handles {
            (*h)();
        }

        println!("End Comment Full View -------------------------- ");
    }
}
