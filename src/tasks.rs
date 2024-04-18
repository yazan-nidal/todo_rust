use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub status: TaskStatus,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}
