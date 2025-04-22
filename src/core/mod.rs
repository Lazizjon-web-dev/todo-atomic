use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use crate::store::{Store, memory::InMemory};
use crate::core::task::{Task, TaskID};

pub mod task;

pub struct ToDoList<S: Store = InMemory> {
    store: S,
    id_gen: AtomicUsize,
}

impl<S: Store> ToDoList<S> {
    pub fn new(store: S) -> Self {
        Self {
            store,
            id_gen: AtomicUsize::new(0),
        }
    }
    pub fn add(&self, desc: String) -> TaskID {
        let id = self.id_gen.fetch_add(1, SeqCst);
        let task = Task {
            id: TaskID(id),
            description: desc,
        };
        self.store.insert(task).unwrap();
        TaskID(id)
    }

    pub fn remove(&self, id: TaskID) -> Result<(), String> {
        // Check if the task exists before removing it
        let task = self.store.get(id.clone()).unwrap();
        if task.is_none() {
            return Err(format!("Task with ID {} not found", id.0));
        }
        // Remove the task from the store
        self.store.remove(id.clone()).unwrap();
        // Check if the task was removed successfully
        let task = self.store.get(id.clone()).unwrap();
        if task.is_some() {
            return Err(format!("Failed to remove task with ID {:?}", id.0));
        }
        Ok(())
    }

    pub fn get(&self, id: TaskID) -> Result<Option<Task>, String> {
        self.store.get(id)
    }

    pub fn get_all(&self) -> Result<Vec<Task>, String> {
        self.store.get_all()
    }
    
}