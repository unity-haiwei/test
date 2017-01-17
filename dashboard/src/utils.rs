use std::process::{Command};

use chrono::{DateTime, UTC};
use bson;
use bson::Document;
use bson::oid::ObjectId;
use config;


pub fn executes_commands(commands: &Vec<&'static str>, start: Option<DateTime<UTC>>, end: Option<DateTime<UTC>>) {
    for c in commands {
        println!("-----------------------------------------------------");
        println!("Execute Command: python easurement.py {} -start {:?} -end {:?}", c, start, end);

        let mut output = Command::new("python");
        output.env("SC_ENV", "onebox");
        output.current_dir(config::SCRIPTS_PATH);
        output.arg("measurement.py");
        output.arg(c);

        if start != None {
            output.arg("-start").arg(start.unwrap().format("%Y-%m-%d").to_string());
        }
        if end != None {
            output.arg("-end").arg(end.unwrap().format("%Y-%m-%d").to_string());
        }

        let result = output.output().expect(format!("failed to execute command").as_str());

        println!("Status: {}", result.status);
        println!("Stdout: {}", String::from_utf8_lossy(&result.stdout));
        println!("Stderr: {}", String::from_utf8_lossy(&result.stderr));
    }
}

pub fn get_obj_id(doc: &Document) -> ObjectId {
    bson::from_bson::<ObjectId>(doc.get("_id").unwrap().clone()).unwrap()
}
