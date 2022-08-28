// veri tabanındaki task_state isimli tipin kod tarafındaki iz düşümü olan enum yapısıdır

#[derive(sqlx::Type, Debug, Clone, Eq, PartialEq)]
#[sqlx(type_name = "task_state")]
#[sqlx(rename_all = "lowercase")]
pub enum TaskState {
    Ready,
    InProgress,
    Completed,
}

// enum sabitini Postgresql tarafına port etmek için kullanılır
sqlb::bindable!(TaskState);
