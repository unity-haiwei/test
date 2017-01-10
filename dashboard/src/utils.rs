use std::process::{Command};

use config;


pub fn executes_commands(commands: &Vec<&'static str>) {
    for c in commands {
        println!("-----------------------------------------------------");
        println!("Execute Command: python easurement.py {}", c);

        let output = Command::new("python")
            .env("SC_ENV", "onebox")
            .current_dir(config::SCRIPTS_PATH)
            .arg("measurement.py")
            .arg(c)
            .output()
            .expect(format!("failed to execute command").as_str());

        println!("Status: {}", output.status);
        println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
