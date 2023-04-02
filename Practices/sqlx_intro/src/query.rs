use crate::model::Category;
use sqlx::{Pool, Postgres};

pub async fn get_categories(pool: Pool<Postgres>) -> Vec<Category> {
    sqlx::query_as!(Category, "select * from categories order by name")
        .fetch_all(&pool)
        .await
        .expect("sorgu başarısız")
}
