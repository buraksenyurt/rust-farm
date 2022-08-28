use crate::model::custom_error::Error;
use crate::model::database::Db;
use crate::model::task_state::TaskState;

#[derive(sqlx::FromRow, Debug)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub state: TaskState,
}

#[derive(Clone)]
pub struct TaskDao {
    pub user_id: Option<i64>,
    pub title: Option<String>,
}

pub struct TaskMac; // Mac = Model Access Controller

impl TaskMac {
    pub async fn get_all(db: &Db) -> Result<Vec<Task>, Error> {
        let sql = "SELECT id,user_id,title,state FROM task ORDER By id DESC";
        let query = sqlx::query_as(&sql);
        let task_list = query.fetch_all(db).await?;

        Ok(task_list)
    }

    pub async fn create(db: &Db, payload: TaskDao) -> Result<Task, Error> {
        let sql =
            "INSERT INTO task (user_id,title) VALUES ($1,$2) returning id,user_id,title,state";
        let query = sqlx::query_as::<_, Task>(&sql)
            .bind(payload.user_id)
            .bind(payload.title);

        let created_task = query.fetch_one(db).await?;

        Ok(created_task)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::database::init;
    use crate::model::task::{TaskDao, TaskMac};
    use crate::model::task_state::TaskState;

    #[tokio::test]
    async fn should_get_all_returns_more_than_one_task() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        let result = TaskMac::get_all(&db).await?;
        assert!(result.len() > 0, "Görevler listesi");

        Ok(())
    }

    #[tokio::test]
    async fn should_create_task_works_with_some_datas() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;

        let candidate_task = TaskDao {
            user_id: Some(4835),
            title: Some(String::from("Rust programlama için bir saat çalış")),
        };

        let created_task = TaskMac::create(&db, candidate_task.clone()).await?;
        assert!(created_task.id >= 1);
        assert_eq!(candidate_task.user_id.unwrap(), created_task.user_id);
        assert_eq!(candidate_task.title.unwrap(), created_task.title);
        assert_eq!(TaskState::Ready, created_task.state);
        Ok(())
    }
}
