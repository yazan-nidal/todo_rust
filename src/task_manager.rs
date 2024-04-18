use crate::tasks::{Task, TaskStatus};
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

pub trait TaskManager {
    fn add_task(&mut self, name: &str) -> Result<Uuid, Box<dyn Error>>;
    fn get_tasks(&self) -> Vec<Task>;
    fn filter_tasks_by_status(&self, status: TaskStatus) -> Vec<Task>;
    fn get_task_by_id(&self, id: Uuid) -> Option<Task>;
    fn move_task(&mut self, id: Uuid, new_status: TaskStatus) -> Option<TaskStatus>;
    fn update_task_name(&mut self, id: Uuid, new_name: &str) -> Option<()>;
    fn delete_task(&mut self, id: Uuid, force: bool) -> Option<()>;
}

pub struct InMemoryTaskManager {
    tasks: HashMap<Uuid, Task>,
}

impl InMemoryTaskManager {
    pub fn new() -> Self {
        InMemoryTaskManager {
            tasks: HashMap::new(),
        }
    }
}

impl TaskManager for InMemoryTaskManager {
    fn add_task(&mut self, name: &str) -> Result<Uuid, Box<dyn Error>> {
        let task_id = Uuid::new_v4();
        let task = Task {
            id: task_id.clone(),
            name: name.to_string(),
            status: TaskStatus::Todo,
        };
        self.tasks.insert(task_id.clone(), task);
        Ok(task_id.clone())
    }

    fn get_tasks(&self) -> Vec<Task> {
        self.tasks.values().cloned().collect()
    }

    fn filter_tasks_by_status(&self, status: TaskStatus) -> Vec<Task> {
        self.tasks
            .values()
            .cloned()
            .filter(|task| task.status == status)
            .collect()
    }

    fn get_task_by_id(&self, id: Uuid) -> Option<Task> {
        self.tasks.get(&id).cloned()
    }

    fn move_task(&mut self, id: Uuid, new_status: TaskStatus) -> Option<TaskStatus> {
        if let Some(task) = self.tasks.get_mut(&id) {
            let current_status = task.status.clone();
            match (current_status.clone(), new_status.clone()) {
                (TaskStatus::Todo, TaskStatus::Doing) => {
                    task.status = TaskStatus::Doing;
                }
                (TaskStatus::Doing, TaskStatus::Done) => {
                    task.status = TaskStatus::Done;
                }
                (TaskStatus::Done, TaskStatus::Doing) => {
                    task.status = TaskStatus::Doing;
                }
                (TaskStatus::Done, TaskStatus::Todo) => {
                    task.status = TaskStatus::Todo;
                }
                (TaskStatus::Doing, TaskStatus::Todo) => {
                    task.status = TaskStatus::Todo;
                }

                //no change in status
                (TaskStatus::Todo, TaskStatus::Todo) => {}
                (TaskStatus::Doing, TaskStatus::Doing) => {}
                (TaskStatus::Done, TaskStatus::Done) => {}

                _ => {
                    println!(
                        "Invalid status transition from {:?} to {:?}",
                        current_status, new_status
                    );
                    return None;
                }
            }
            Some(task.status.clone())
        } else {
            println!("Task with ID {} not found", id);
            None
        }
    }

    fn update_task_name(&mut self, id: Uuid, new_name: &str) -> Option<()> {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.name = new_name.to_string();
            Some(())
        } else {
            println!("Task with ID {} not found", id);
            None
        }
    }

    fn delete_task(&mut self, id: Uuid, force: bool) -> Option<()> {
        if let Some(task) = self.tasks.get(&id.clone()) {
            if task.status == TaskStatus::Done || force {
                self.tasks.remove(&id.clone());
                Some(())
            } else {
                println!(
                    "Task with ID {} cannot be deleted. Move it to Deon or use -f to force delete.",
                    id
                );
                None
            }
        } else {
            println!("Task with ID {} not found", id);
            None
        }
    }
}
