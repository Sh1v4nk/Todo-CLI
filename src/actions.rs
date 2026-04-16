use std::io::{self, Write};

#[derive(Debug)]
pub struct Task {
    id: usize,
    title: String,
    description: String,
    done: bool,
}

pub fn read_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?; // ensure prompt shows immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input)?; // mutable borrow

    Ok(input.trim().to_string())
}

pub fn add_task(tasks: &mut Vec<Task>, title: String, description: String) {
    let id = tasks.last().map_or(1, |t| t.id + 1); // if empty, start at 1; else increment last id
    tasks.push(Task {
        id,
        title,
        description,
        done: false,
    });
    println!("✓ Task added.");
}

pub fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks yet. Add one!");
        return;
    }

    println!("\n--- Your Tasks ---");
    for task in tasks {
        let status = if task.done {
            "✓ Completed"
        } else {
            "○ Incomplete"
        };
        println!(
            "[{}]\n Task ID: {}\n Task Title: {}\n Task Description: {}\n",
            status, task.id, task.title, task.description
        );
    }
    println!("------------------\n");
}

pub fn delete_task(tasks: &mut Vec<Task>, id: usize) {
    // can use tasks.retain too, but it scans whole vector.
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        println!("✓ Task {} deleted.", id);
    } else {
        println!("✗ No task with id {}.", id);
    }
}

pub fn toggle_task(tasks: &mut Vec<Task>, id: usize) {
    for task in tasks.iter_mut() {
        if task.id == id {
            task.done = !task.done;
            let state = if task.done { "done" } else { "undone" };
            println!("✓ Task {} marked as {}.", id, state);
            return;
        }
    }
    println!("✗ No task with id {}.", id);
}
