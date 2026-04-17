use crate::task::Task;
use std::fs;

const FILE: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    match fs::read_to_string(FILE) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(FILE, json).expect("Failed to write tasks.json");
}
