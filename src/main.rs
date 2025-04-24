use rustyline::DefaultEditor;
use todo_atomic::core::{ToDoList, task::TaskID};
use todo_atomic::store::memory::InMemory;
fn main() {
    // Initialize the command line editor
    let mut rl = DefaultEditor::new().unwrap();

    // Create a new ToDoList instance
    let to_do_list = ToDoList::new(InMemory::new());

    loop {
        // Prompt the user for input
        let readline = rl.readline(">> ");
        // Read a line from the command line
        match readline {
            // If the line is empty, continue to the next iteration
            Ok(line) if line.trim() == "" => {
                println!("Empty line, please enter a command");
                continue;
            }
            // If the line is not empty, process the command
            Ok(line) => {
                // Trim the line to remove leading and trailing whitespace
                let line = line.trim();
                // Check the first word of the line to determine the command
                match line.split_whitespace().nth(0).unwrap_or("") {
                    // If the command is "exit", break the loop
                    "\\exit" => {
                        println!("Exiting...");
                        break;
                    }
                    // If the command is "search", search for a task by ID
                    "\\search" => {
                        // Get the task ID from the line
                        let task_id = line.split_whitespace().nth(1).unwrap_or("");
                        // Parse the task ID to a usize
                        match task_id.parse::<usize>() {
                            // If parsing is successful, get the task from the ToDoList
                            Ok(id) => {
                                // Get the task from the ToDoList
                                let task = to_do_list.get(TaskID(id));
                                // Match the result of the get operation
                                match task {
                                    Ok(Some(task)) => println!("Task ID: {}, Description: {:?}", task.id.0, task.description),
                                    Ok(None) => println!("Task not found"),
                                    Err(e) => println!("Error: {}", e),
                                }
                            }
                            // If parsing fails, print an error message
                            Err(_) => {
                                println!("Invalid task ID");
                            }
                        }
                    }
                    // If the command is "add", add a new task
                    "\\add" => {
                        // Get the task description from the line
                        let task_description = line.trim_start_matches("\\add ").trim();
                        // Check if the task description is too long
                        if task_description.len() > 100 {
                            println!("Task description is too long (max 100 characters)");
                            continue;
                        }
                        // Check if the task description contains invalid characters
                        if task_description.chars().any(|c| !c.is_alphanumeric() && c != ' ') {
                            println!("Task description contains invalid characters");
                            continue;
                        }
                        
                        // Check if the task description is empty
                        if task_description.is_empty() {
                            println!("Task description cannot be empty");
                            continue;
                        }
                        // Add the task to the ToDoList
                        let task_id = to_do_list.add(task_description.to_string());
                        // Print the ID of the added task
                        println!("Added task with ID: {}", task_id.0);
                    }
                    // If the command is "remove", remove a task by ID
                    "\\remove" => {
                        // Get the task ID from the line
                        let task_id = line.split_whitespace().nth(1).unwrap_or("");
                        // Parse the task ID to a usize
                        match task_id.parse::<usize>() {
                            // If parsing is successful, remove the task from the ToDoList
                            Ok(id) => {
                                // Remove the task from the ToDoList
                                match to_do_list.remove(TaskID(id)) {
                                    Ok(_) => println!("Task removed successfully"),
                                    Err(e) => println!("Error: {}", e),
                                }
                            }
                            // If parsing fails, print an error message
                            Err(_) => {
                                println!("Invalid task ID");
                            }
                        }
                    }
                    // If the command is "list", list all tasks
                    "\\list" => {
                        // Get all tasks from the ToDoList
                        let tasks = to_do_list.get_all();
                        // Match the result of the get_all operation
                        match tasks {
                            // If successful, print the tasks
                            Ok(tasks) => {
                                for task in tasks {
                                    println!("Task ID: {:?}, Description: {:?}", task.id.0, task.description);
                                }
                            }
                            // If an error occurs, print the error message
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
                    // If the command is "help", print the available commands
                    "\\help" => {
                        println!("Available commands:");
                        println!("\\add <task_description> - Add a new task");
                        println!("\\remove <task_id> - Remove a task");
                        println!("\\search <task_id> - Search for a task by ID");
                        println!("\\list - List all tasks");
                        println!("\\exit - Exit the program");
                    }
                    // If the command is not recognized, print an error message
                    &_ => {
                        println!("Unknown command: {}", line);
                    }
                }
            }
            // If an error occurs while reading the line, print an error message
            Err(_) => {
                println!("Error reading line");
                break;
            }
        }
    }
}