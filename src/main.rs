use rustyline::DefaultEditor;
use todo_atomic::core::{ToDoList, task::{Task, TaskID}};
use todo_atomic::store::memory::InMemory;
fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline(">> ");
        let to_do_list = ToDoList::new(InMemory::new());

        match readline {
            Ok(line) if line.trim() == "" => {
                println!("Empty line, please enter a command");
                continue;
            }
            Ok(line) => {
                let line = line.trim();
                match line {
                    "\\exit" => {
                        println!("Exiting...");
                        break;
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