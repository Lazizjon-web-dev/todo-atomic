pub mod memory;
use crate::core::task::{Task, TaskID};

pub trait Store {
    fn insert(&self, task: Task) -> Result<(), String>;
    fn remove(&self, id: TaskID) -> Result<(), String>;
    fn get(&self, id: TaskID) -> Result<Option<Task>, String>;
    fn get_all(&self) -> Result<Vec<Task>, String>;
}