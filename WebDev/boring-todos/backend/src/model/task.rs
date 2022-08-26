use crate::model::database::Db;

#[derive(sqlx::FromRow, Debug)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
}

pub struct TaskMac; // Mac = Model Access Controller

impl TaskMac {
    pub async fn get_all(db: &Db) -> Result<Vec<Task>, sqlx::Error> {
        let sql = "SELECT id,user_id,title,state FROM task ORDER By id DESC";
        let query = sqlx::query_as(&sql);
        let task_list = query.fetch_all(db).await?;

        Ok(task_list)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::database::init;
    use crate::model::task::TaskMac;

    #[tokio::test]
    async fn should_get_all_returns_more_than_one_task() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        let result = TaskMac::get_all(&db).await?;
        assert!(result.len() > 0, "Görevler listesi");

        Ok(())
    }
}
