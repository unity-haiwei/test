use core::ops::Add;

use bson::Document;
use chrono::{DateTime, UTC};
use chrono::duration::Duration;

use config;
use utils;
use db::{self, DBTrait};
use Runtime;
use super::LogicTrait;

#[derive(Clone)]
pub enum JobType {
    FullTime,
    #[allow(dead_code)]
    PartTime,
    #[allow(dead_code)]
    Internship,
    Freelance,
}

#[derive(Clone)]
pub struct Job<'a> {
    runtime: &'a Runtime,
    db: db::Job,
    db_team: db::Team,
    users: Vec<Document>,
}


impl<'a> Job<'a> {
    pub fn generate_job(&self, amount: u32, user_index: &mut usize, created_time: DateTime<UTC>, job_type: JobType) {
        for _ in 0..amount {
            let user = &self.users[*user_index];
            let user_id = utils::get_obj_id(&user);

            let (team, job_type, msg_type) = match job_type {
                JobType::Freelance => (self.db_team.get_default_team(), "freelance", "Task"),
                _ => (self.db_team.get_not_default_team(), "fullTime", "Job"),
            };
            let team_id = utils::get_obj_id(&team);

            let job_id = self.db.create(user_id.clone(), job_type, team_id.clone(), created_time);

            println!("Created {}({}), User({})", msg_type, job_id, user_id);

            *user_index += 1;
        }
    }

    pub fn remove_all(&self) {
        self.db.remove_all();
    }
}

impl<'a> LogicTrait<'a> for Job<'a> {

    fn new(r: &'a Runtime) -> Job<'a> {
        Job {
            runtime: r,
            db: db::Job::new(),
            db_team: db::Team::new(),
            users: db::User::new().all(),
        }
    }

    fn run(&self) {
        println!("Job and Task Full View -------------------------- {}", UTC::now());
        println!("Create 2 copies of the data.");

        let commands_setup = vec![config::COMMAND_DROP, config::COMMAND_SETUP];
        let commands_sync_job = vec![config::COMMAND_TOTAL_JOB,
                                     config::COMMAND_NEW_JOB,
                                     config::COMMAND_TOTAL_JOB_USER_PERCENT,
                                     config::COMMAND_DAILY_JOB_USER_PERCENT];

        let commands_sync_task = vec![config::COMMAND_TOTAL_TASK,
                                      config::COMMAND_NEW_TASK,
                                      config::COMMAND_TOTAL_TASK_USER_PERCENT,
                                      config::COMMAND_DAILY_TASK_USER_PERCENT];

        println!("Clear all data --------------------------------------");
        self.remove_all();
        utils::executes_commands(self.runtime.script_path, &commands_setup, None, None);

        let begin_time = UTC::now().add(Duration::days(-5));
        let mut user_index = 0;
        let mut created_time = begin_time.clone();
        println!("Move time window to {}", created_time);

        self.generate_job(1, &mut user_index, created_time, JobType::FullTime);
        self.generate_job(1, &mut user_index, created_time, JobType::Freelance);

        created_time = created_time.add(Duration::days(1));
        println!("Move time window to {}", created_time);

        self.generate_job(2, &mut user_index, created_time, JobType::FullTime);
        self.generate_job(2, &mut user_index, created_time, JobType::Freelance);

        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_job,
                                 Some(begin_time),
                                 Some(created_time.add(Duration::days(1))));
        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_task,
                                 Some(begin_time),
                                 Some(created_time.add(Duration::days(1))));

        created_time = created_time.add(Duration::days(2));
        println!("Move time window to {}", created_time);

        self.generate_job(2, &mut user_index, created_time, JobType::FullTime);
        self.generate_job(3, &mut user_index, created_time, JobType::Freelance);

        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_job,
                                 Some(created_time.add(Duration::days(-1))),
                                 Some(created_time.add(Duration::days(1))));
        utils::executes_commands(self.runtime.script_path,
                                 &commands_sync_task,
                                 Some(created_time.add(Duration::days(-1))),
                                 Some(created_time.add(Duration::days(1))));

    }
}
