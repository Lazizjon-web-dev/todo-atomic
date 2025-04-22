#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskID,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskID(pub usize);