use crate::model::{Category, Product};
use sqlx::{Pool, Postgres};

pub async fn insert_product(pool: &Pool<Postgres>, p: Product) -> Product {
    sqlx::query_as!(
        Product,
        r#"insert into products (title,category_id,unit_price) values ($1,$2,$3) returning id,title,category_id,unit_price as "unit_price!""#,
        p.title,p.category_id,p.unit_price
    )
    .fetch_one(pool)
    .await
    .expect("insert product sorgusu başarısız")
}

pub async fn insert_category(pool: &Pool<Postgres>, c: Category) -> Category {
    let result = sqlx::query_as!(
        Category,
        "insert into categories (title) values ($1) returning *",
        c.title
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(inserted) => inserted,
        Err(_) => c,
    }
}

pub async fn apply_discount(pool: &Pool<Postgres>, id: i64, rate: f32) -> Product {
    let discount = rate / 100.;
    sqlx::query_as!(
        Product,
        r#"update products set unit_price = unit_price - (unit_price * $1) where id = $2 returning id,title,category_id,unit_price as "unit_price!""#,
        discount,
        id
    )
        .fetch_one(pool)
        .await
        .expect("update product sorgusu başarısız")
}

pub async fn delete_all_products(pool: &Pool<Postgres>) -> bool {
    let result = sqlx::query_as!(
        Product,
        r#"delete from products returning id,title,category_id,unit_price as "unit_price!""#
    )
    .fetch_all(pool)
    .await;
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}
