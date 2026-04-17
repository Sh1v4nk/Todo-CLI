use crate::task::{Priority, Task};
use colored::Colorize;

fn count_stats(tasks: &[&Task]) -> (usize, usize) {
    let done = tasks.iter().filter(|t| t.done).count();
    (done, tasks.len().saturating_sub(done))
}

fn wrap_words(text: &str, width: usize) -> Vec<String> {
    if text.trim().is_empty() {
        return vec![];
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let additional = if current.is_empty() {
            word.len()
        } else {
            word.len() + 1
        };

        if current.len() + additional > width && !current.is_empty() {
            lines.push(current);
            current = word.to_string();
        } else {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn priority_badge(p: &Priority) -> String {
    let base = format!("[{}]", p.label());

    match p {
        Priority::High => base.red().bold().to_string(),
        Priority::Medium => base.yellow().to_string(),
        Priority::Low => base.cyan().to_string(),
    }
}

pub fn print_tasks(tasks: &[&Task]) {
    let (done, pending) = count_stats(tasks);

    println!();
    println!(
        "{}  {}",
        "TASKS".bold(),
        format!(
            "{} total · {} pending · {} done",
            tasks.len(),
            pending,
            done
        )
        .dimmed()
    );
    println!();

    for task in tasks {
        print_single_task(task);
        println!(); // spacing between tasks
    }
}

fn print_single_task(task: &Task) {
    // Status icon
    let icon = if task.done {
        "✓".green().bold()
    } else {
        "○".dimmed()
    };

    let id = format!("#{}", task.id).dimmed();
    let priority = priority_badge(&task.priority);

    // Title styling
    let title = if task.done {
        task.title.strikethrough().dimmed()
    } else {
        task.title.bold()
    };

    // ── Line 1: Title row
    println!("{}  {} {}  {}", icon, id, priority, title);

    // ── Line 2: Time
    let time = task
        .created_at
        .format("%a %b %d %I:%M:%S %p")
        .to_string()
        .dimmed();
    println!("      {}", time);

    // ── Line 3: Description
    let lines = wrap_words(&task.description, 60);

    for line in lines {
        println!("      {}", line.dimmed());
    }
}
