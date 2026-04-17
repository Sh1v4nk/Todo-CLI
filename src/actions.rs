use crate::storage::save_tasks;
use crate::task::{Priority, Task};
use chrono::Local;
use colored::Colorize;

pub fn add_task(tasks: &mut Vec<Task>, title: String, description: String, priority: Priority) {
    // .last() returns Option<&Task>
    // map_or(default, closure) — if None (empty vec) use 1, else increment last id
    let id = tasks.last().map_or(1, |t| t.id + 1);

    tasks.push(Task {
        id,
        title,
        description,
        done: false,
        priority,
        created_at: Local::now(),
    });

    save_tasks(tasks);
    println!("{} Task added", "✓".green().bold());
}

pub fn delete_task(tasks: &mut Vec<Task>, id: usize) {
    // position() short-circuits — stops at first match, unlike retain() which scans all
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        save_tasks(tasks);
        println!("{} Task {} deleted", "✓".green().bold(), id);
    } else {
        println!("{} No task with id {}", "✗".red().bold(), id);
    }
}

pub fn toggle_task(tasks: &mut Vec<Task>, id: usize) {
    if let Some(i) = tasks.iter().position(|t| t.id == id) {
        tasks[i].done = !tasks[i].done; // mutate via index borrow ends here
        let state = if tasks[i].done { "done" } else { "undone" };

        println!("{} Task {} marked {}", "✓".green().bold(), id, state);
        save_tasks(tasks); // no active mutable borrow in scope
    } else {
        println!("{} No task with id {}", "✗".red().bold(), id);
    }
}

pub fn list_filtered(tasks: &[Task], filter: Option<Priority>) {
    let mut view: Vec<&Task> = tasks
        .iter()
        .filter(|t| match &filter {
            Some(p) => &t.priority == p,
            None => true,
        })
        .collect();

    // sort newest first DateTime<Local> implements Ord so .cmp() works directly
    view.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    if view.is_empty() {
        println!("{}", "No tasks found".dimmed());
        return;
    }

    crate::display::print_tasks(&view);
}
