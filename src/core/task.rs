#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskID,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TaskID(usize);