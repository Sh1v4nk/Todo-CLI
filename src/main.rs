use colored::Colorize;
use std::io::{self, Write};

mod actions;
mod display;
mod storage;
mod task;

use task::Priority;

fn read_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn print_menu() {
    println!();

    println!(
        "{}  {}",
        "TODO CLI".bold(),
        "✦ A minimal task manager".dimmed()
    );

    println!();

    println!("{} Add task", "1.".cyan().bold());
    println!("{} View tasks", "2.".cyan().bold());
    println!("{} Filter by priority", "3.".cyan().bold());
    println!("{} Delete task", "4.".cyan().bold());
    println!("{} Toggle done", "5.".cyan().bold());
    println!("{} Exit", "6.".cyan().bold());

    println!();
}

fn main() -> Result<(), io::Error> {
    let mut tasks = storage::load_tasks();

    loop {
        print_menu();

        let choice = read_input("› Choose: ")?;

        match choice.as_str() {
            "1" => {
                let title = read_input("› Title: ")?;
                let description = read_input("› Description: ")?;
                let p = read_input("› Priority (high/med/low) [med]: ")?;

                actions::add_task(&mut tasks, title, description, Priority::from_str(&p));
            }

            "2" => actions::list_filtered(&tasks, None),

            "3" => {
                let p = read_input("› Filter (high/med/low): ")?;
                actions::list_filtered(&tasks, Some(Priority::from_str(&p)));
            }

            "4" => {
                let input = read_input("› Task ID to delete: ")?;
                match input.parse::<usize>() {
                    Ok(id) => actions::delete_task(&mut tasks, id),
                    Err(_) => println!("{} Invalid number", "✗".red().bold()),
                }
            }

            "5" => {
                let input = read_input("› Task ID to toggle: ")?;
                match input.parse::<usize>() {
                    Ok(id) => actions::toggle_task(&mut tasks, id),
                    Err(_) => println!("{} Invalid number", "✗".red().bold()),
                }
            }

            "6" => {
                println!();
                println!("{}", "Goodbye ✦\n".dimmed());
                break;
            }

            _ => println!("{} Invalid choice", "✗".red().bold()),
        }
    }

    Ok(())
}
