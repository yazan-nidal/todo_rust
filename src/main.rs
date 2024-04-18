use std::error::Error;
use std::io::Write;
use std::{fs, io};

use todo_app::commands::*;
use todo_app::task_manager::*;
use toml::Value;

fn read_app_name_from_cargo_toml() -> Option<String> {
    if let Ok(cargo_toml_content) = fs::read_to_string("Cargo.toml") {
        if let Ok(value) = cargo_toml_content.parse::<Value>() {
            if let Some(table) = value.as_table() {
                if let Some(package) = table.get("package") {
                    if let Some(name) = package.get("name") {
                        if let Some(name_str) = name.as_str() {
                            return Some(name_str.to_string().replace('_', " "));
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read application name from Cargo.toml
    if let Some(app_name) = read_app_name_from_cargo_toml() {
        let mut manager: Box<dyn TaskManager> = Box::new(InMemoryTaskManager::new());

        loop {
            let mut input = String::new();
            print!("({}) > ", app_name);
            io::stdout().flush().unwrap();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let trimmed_input = input.trim();

            if trimmed_input.is_empty() {
                continue;
            }

            let parts: Vec<&str> = trimmed_input.split_whitespace().collect();
            let command = parts[0];
            let parts = &parts[1..];

            // Process user input using match statement
            match command.to_lowercase().as_str() {
                "add" => handle_add_command(&mut manager, &parts.join(" "))?,

                "show" => handle_show_command(&mut manager, &parts)?,

                "move" => handle_move_command(&mut manager, &parts)?,

                "update" => handle_update_command(&mut manager, &parts)?,

                "delete" => handle_delete_command(&mut manager, &parts)?,

                "help" => handle_help_command(),

                "exit" => {
                    // Exit the program
                    println!("Exiting the {} task manager. Goodbye!", app_name);
                    break (Ok(()));
                }
                _ => {
                    println!("Invalid command. Try again.");
                }
            }
        }
    } else {
        println!("Invalid command. Try again.");
        Ok(())
    }
}
