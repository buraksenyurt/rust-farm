#[derive(sqlx::Type, Debug, Clone, Eq, PartialEq)]
#[sqlx(type_name = "task_state")]
#[sqlx(rename_all = "lowercase")]
pub enum TaskState {
    Ready,
    InProgress,
    Completed,
}
