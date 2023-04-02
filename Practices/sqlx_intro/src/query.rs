use crate::model::{Category, Product};
use sqlx::{Pool, Postgres};

pub async fn get_categories(pool: &Pool<Postgres>) -> Vec<Category> {
    sqlx::query_as!(Category, "select * from categories order by title")
        .fetch_all(pool)
        .await
        .expect("sorgu başarısız")
}

// Alınan f32, Option hatası nedeniyle unit_price as "unit_price!" şeklinde bir kullanım söz konusu.
pub async fn get_product_by_id(pool: &Pool<Postgres>, id: i64) -> Product {
    sqlx::query_as!(
        Product,
        r#"select id,title,category_id,unit_price as "unit_price!" from products where id = $1"#,
        id
    )
    .fetch_one(pool)
    .await
    .expect("sorgu başarısız")
}

pub async fn get_products_by_category(pool: &Pool<Postgres>, category_id: i64) -> Vec<Product> {
    sqlx::query_as!(
        Product,
        r#"select id,title,category_id,unit_price as "unit_price!" from products where category_id = $1"#,
        category_id
    )
    .fetch_all(pool)
    .await
    .expect("sorgu başarısız")
}
