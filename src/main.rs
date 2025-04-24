use rustyline::DefaultEditor;
use todo_atomic::core::{ToDoList, task::{Task, TaskID}};
use todo_atomic::store::memory::InMemory;
fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let to_do_list = ToDoList::new(InMemory::new());
    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) if line.trim() == "" => {
                println!("Empty line, please enter a command");
                continue;
            }
            Ok(line) => {
                let line = line.trim();
                match line.split_whitespace().nth(0).unwrap_or("") {
                    "\\exit" => {
                        println!("Exiting...");
                        break;
                    }
                    "\\search" => {
                        let task_id = line.split_whitespace().nth(1).unwrap_or("");
                        println!("{}", task_id);
                        match task_id.parse::<usize>() {
                            Ok(id) => {
                                let task = to_do_list.get(TaskID(id));
                                match task {
                                    Ok(Some(task)) => println!("Task: {:?}", task),
                                    Ok(None) => println!("Task not found"),
                                    Err(e) => println!("Error: {}", e),
                                }
                            }
                            Err(_) => {
                                println!("Invalid task ID");
                            }
                        }
                    }
                    "\\add" => {
                        let task_description = line.trim_start_matches("\\add ").trim();
                        let task_id = to_do_list.add(task_description.to_string());
                        println!("Added task with ID: {:?}", task_id);
                    }
                    "\\remove" => {
                        let task_id = line.split_whitespace().nth(1).unwrap_or("");
                        match task_id.parse::<usize>() {
                            Ok(id) => {
                                match to_do_list.remove(TaskID(id)) {
                                    Ok(_) => println!("Task removed successfully"),
                                    Err(e) => println!("Error: {}", e),
                                }
                            }
                            Err(_) => {
                                println!("Invalid task ID");
                            }
                        }
                    }
                    "\\list" => {
                        let tasks = to_do_list.get_all();
                        match tasks {
                            Ok(tasks) => {
                                for task in tasks {
                                    println!("Task ID: {:?}, Description: {:?}", task.id, task.description);
                                }
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
                    "\\help" => {
                        println!("Available commands:");
                        println!("\\add <task_description> - Add a new task");
                        println!("\\remove <task_id> - Remove a task");
                        println!("\\search <task_id> - Search for a task by ID");
                        println!("\\list - List all tasks");
                        println!("\\exit - Exit the program");
                    }
                    &_ => {
                        println!("Unknown command: {}", line);
                    }
                }
            }
            Err(_) => {
                println!("Error reading line");
                break;
            }
        }
    }
}