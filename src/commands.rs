use crate::task_manager::TaskManager;
use crate::tasks::*;
use std::error::Error;
use uuid::Uuid;

pub fn display_tasks(tasks: Vec<Task>, status: &str) {
    if tasks.is_empty() {
        if status.is_empty() {
            println!("Empty List!");
        } else {
            println!("Empty {} List!", status);
        }
        return;
    }

    for task in &tasks {
        println!(
            "id:\t{},\tname:\t{},\tstatus:\t{:?}",
            task.id, task.name, task.status
        );
    }
}

pub fn handle_add_command(
    manager: &mut Box<dyn TaskManager>,
    task_name: &str,
) -> Result<(), Box<dyn Error>> {
    if !task_name.is_empty() {
        let task_id = manager.add_task(task_name)?;
        println!("new task:{:?}, was add", task_id);
    } else {
        println!("please insert name of task ");
    }
    Ok(())
}

pub fn handle_show_command(
    manager: &mut Box<dyn TaskManager>,
    args: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    match args.len() {
        0 => {
            let tasks = manager.get_tasks();
            display_tasks(tasks, "");
        }

        1 => match args[0].to_lowercase().as_str() {
            "*" => {
                let tasks = manager.get_tasks();
                display_tasks(tasks, "");
            }
            "todo" => {
                let filtered_tasks = manager.filter_tasks_by_status(TaskStatus::Todo);
                display_tasks(filtered_tasks, "todo");
            }
            "doing" => {
                let filtered_tasks = manager.filter_tasks_by_status(TaskStatus::Doing);
                display_tasks(filtered_tasks, "doing");
            }
            "done" => {
                let filtered_tasks = manager.filter_tasks_by_status(TaskStatus::Done);
                display_tasks(filtered_tasks, "done");
            }

            arg => {
                let id = match uuid::Uuid::parse_str(arg) {
                    Ok(parsed_id) => parsed_id,
                    Err(_) => {
                        println!("Invalid UUID: {}", arg);
                        return Ok(()); // Gracefully handle invalid UUID without error
                    }
                };

                if let Some(task) = manager.get_task_by_id(id) {
                    display_tasks(vec![task], "");
                } else {
                    println!("Task not found for ID: {}", id);
                }
            }
        },

        _ => println!("Invalid 'show' command usage."),
    }
    Ok(())
}

pub fn handle_move_command(
    manager: &mut Box<dyn TaskManager>,
    args: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    match args.len() {
        2 => {
            let id = match extract_task_id(args) {
                Ok(value) => value,
                Err(value) => return value,
            };

            let result = match args[1].to_lowercase().as_str() {
                "todo" => manager.move_task(id, TaskStatus::Todo),
                "doing" => manager.move_task(id, TaskStatus::Doing),
                "done" => manager.move_task(id, TaskStatus::Done),
                &_ => {
                    println!("unknown status:  {:?}", args[1]);
                    None
                }
            };

            if !result.is_none() {
                println!(
                    "Task:{:?}, its status has been moved to {:?}",
                    id,
                    result.ok_or("")?
                );
            }
        }
        _ => println!("Invalid 'move' command usage."),
    }
    Ok(())
}

pub fn handle_update_command(
    manager: &mut Box<dyn TaskManager>,
    args: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() >= 2 {
        let id = match extract_task_id(args) {
            Ok(value) => value,
            Err(value) => return value,
        };

        let result = manager.update_task_name(id, &args[1..].join(" "));

        if !result.is_none() {
            println!(
                "Task:{:?}, its name has been changed to {:?}",
                id,
                &args[1..].join(" ")
            );
        }
    } else {
        println!("Invalid 'update' command usage.");
    }
    Ok(())
}

pub fn handle_delete_command(
    manager: &mut Box<dyn TaskManager>,
    args: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    match args.len() {
        1 => {
            //delete <task_id>
            let id = match extract_task_id(args) {
                Ok(value) => value,
                Err(value) => return value,
            };

            if let Some(_) = manager.delete_task(id.clone(), false) {
                println!("Task with ID {} deleted successfully.", id);
            }
        }

        2 => {
            //delete <task_id> -f
            let id = match Uuid::parse_str(args[0]) {
                Ok(parsed_id) => parsed_id,
                Err(_) => {
                    println!("Invalid UUID: {}", args[0]);
                    return Ok(());
                }
            };

            if args[1] == "-f" {
                if let Some(_) = manager.delete_task(id.clone(), true) {
                    println!("Task with ID {} force deleted successfully.", id);
                }
            } else {
                println!("Invalid option: {}", args[2]);
            }
        }

        _ => {
            println!("Invalid 'delete' command usage. Try: delete <task_id> or delete <task_id> -f")
        }
    }

    Ok(())
}

pub fn handle_help_command() {
    println!("Commands: <Insensitive>");
    println!("------------------");
    println!("add <task_name>               : Add a new task with the specified name<sensitive>");
    println!("move <task_id> <status>       : Move a task to the specified status (todo -> doing -> done || done -> [doing | todo] || doing -> todo )");
    println!("show [op]                     : show | show * -> Show all tasks || show status ->  Show all tasks in that status  || show task_id  -> show that task");
    println!("delete <task_id> [op]         : delete task_id -> Delete a task with the specified ID if task in Done || delete task_id -f -> force delete for task from any status");
    println!("update <task_id> <new_name>   : Update the name of a task");
    println!("exit");
}

pub fn extract_task_id(args: &[&str]) -> Result<Uuid, Result<(), Box<dyn Error>>> {
    let id = match uuid::Uuid::parse_str(args[0]) {
        Ok(parsed_id) => parsed_id,
        Err(_) => {
            println!("Invalid UUID: {}", args[0]);
            return Err(Ok(()));
        }
    };
    Ok(id)
}
