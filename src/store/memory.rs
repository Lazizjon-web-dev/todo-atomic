use crate::store::Store;
use crate::core::task::{Task, TaskID};
use core::task;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct InMemory {
    tasks: RwLock<HashMap<TaskID, Task>>,
}
impl InMemory {
    pub fn new() -> Self {
        InMemory {
            tasks: RwLock::new(HashMap::new()),
        }
    }
}
impl Store for InMemory {
    fn insert(&self, task: Task) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if tasks.contains_key(&task.id) {
            return Err(format!("Task already exists: {:?}", task.id));
        }
        tasks.insert(task.id.clone(), task);
        Ok(())
    }

    fn remove(&self, id: TaskID) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if tasks.remove(&id).is_none() {
            return Err(format!("Task not found: {:?}", id));
        }
        Ok(())
    }

    fn get(&self, id: TaskID) -> Result<Option<Task>, String> {
        let tasks = self.tasks.read().unwrap();
        match tasks.get(&id) {
            Some(task) => Ok(Some(task.clone())),
            None => Ok(None),
        }
    }

    fn get_all(&self) -> Result<Vec<Task>, String> {
        let tasks = self.tasks.read().unwrap();
        Ok(tasks.values().cloned().collect())
    }
}