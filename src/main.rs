use chrono::prelude::{NaiveDateTime};
use std::vec::Vec;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::str::from_utf8;
use serde_json::Value;
use clap::{Command, Arg};

struct Task {
    name:       String,
    desc:       String,
    do_at:      Option<NaiveDateTime>,
    do_by:      Option<NaiveDateTime>,
    ignore_by:  Option<NaiveDateTime>,
}

struct ProgramData<'a> {
    tasks: Vec<Task>,
    save_path: &'a Path
}

fn load_data(path: &Path) -> ProgramData {
    let mut out = ProgramData {
        tasks: Vec::new(),
        save_path: path
    };
    // Get the "tasks" file in the save path as a PathBuf
    let mut b = PathBuf::from(path);
    b.push("tasks");
    // PathBuf to &Path
    let main_save_file = b.as_path();
    // Interpret the file's json if it exists
    if main_save_file.exists() {
        let mut f = File::open(main_save_file).expect("File exists, but can't be opened!");
        let mut json_data = Vec::new();
        f.read_to_end(&mut json_data);
        let json_base: Value = serde_json::from_str(from_utf8(json_data.as_slice())
                                                .expect("Broken UTF8 Encoding!")
                                           ).expect("Broken JSON!");

        let version: u64 = json_base["version"].as_u64().expect("No version number!");
        if version == 1 {
            for task_json in json_base["tasks"].as_array().expect("No tasks array!") {
                let task_map = task_json.as_object().expect("Task not an object!");
                out.tasks.push(Task {
                    name:       task_map.get("name").expect("Task lacks name!").to_string(),
                    desc:       task_map.get("desc").expect("Task lacks description!").to_string(),
                    do_at:      match task_map.get("do_at") {
                        Some(s) => Some(NaiveDateTime::parse_from_str(&s.to_string(), "%Y %b %d %H:%M")
                                                        .expect("Broken time string!")),
                        None => None
                    },
                    do_by:      match task_map.get("do_by") {
                        Some(s) => Some(NaiveDateTime::parse_from_str(&s.to_string(), "%Y %b %d %H:%M")
                                                        .expect("Broken time string!")),
                        None => None
                    },
                    ignore_by:  match task_map.get("ignore_by") {
                        Some(s) => Some(NaiveDateTime::parse_from_str(&s.to_string(), "%Y %b %d %H:%M")
                                                        .expect("Broken time string!")),
                        None => None
                    }
                });
            }
        } /* more version parsers go here */
        else {panic!("Bad version!")}
    }
    out
}

fn main() {
    let mut data = load_data(Path::new("/home/aprzn/.everything_todo"));
    let matches = Command::new("Everything")
                        .about("A to-do list program")
                        .subcommand(Command::new("add")
                            .alias("new")
                            .arg(Arg::new("name"))
                            .arg(Arg::new("desc"))
                            .arg(Arg::new("due_time"))
                            .arg(Arg::new("do_at_time"))
                        ).get_matches();
}
