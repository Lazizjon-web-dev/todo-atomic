pub struct InMemory {
    tasks: RwLock<HashMap<TaskID, Task>>,
}

impl Store for InMemory {
    fn insert(&self, task: Task) -> Result<(), Error> {
        let mut tasks = self.tasks.write().unwrap();
        if tasks.contains_key(&task.id) {
            return Err(Error::TaskAlreadyExists(task.id))
        }
        tasks.insert(task.id, task);
        Ok(())
    }
    fn get_all(&self) -> Vec<Task> {
        let tasks = self.read().unwrap();
        tasks.values().cloned().collect()
    }
}