use crate::model::Product;
use sqlx::{Pool, Postgres};

pub async fn insert_product(pool: &Pool<Postgres>, p: &Product) -> Product {
    sqlx::query_as!(
        Product,
        r#"insert into products (title,category_id,unit_price) values ($1,$2,$3) returning id,title,category_id,unit_price as "unit_price!""#,
        p.title,p.category_id,p.unit_price
    )
    .fetch_one(pool)
    .await
    .expect("insert sorgusu başarısız")
}
