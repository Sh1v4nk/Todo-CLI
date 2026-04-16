mod actions;

use actions::*;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n=== Todo CLI ===");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Delete task");
        println!("4. Toggle done");
        println!("5. Exit");

        let choice = read_input("Choose: ")?;

        match choice.as_str() {
            "1" => {
                let title = read_input("Task title[few words]: ")?;
                let description = read_input("Task description: ")?;
                add_task(&mut tasks, title, description);
            }
            "2" => list_tasks(&tasks),
            "3" => {
                let input = read_input("Task ID to delete: ")?;
                match input.parse::<usize>() {
                    Ok(id) => delete_task(&mut tasks, id),
                    Err(_) => println!("✗ Please enter a valid number."),
                }
            }
            "4" => {
                let input = read_input("Task ID to toggle: ")?;
                match input.parse::<usize>() {
                    Ok(id) => toggle_task(&mut tasks, id),
                    Err(_) => println!("✗ Please enter a valid number."),
                }
            }
            "5" => {
                println!("Bye!");
                break;
            }
            _ => println!("✗ Invalid choice."),
        }
    }

    Ok(())
}
