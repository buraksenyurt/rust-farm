mod command;
mod model;
mod query;

use crate::command::{apply_discount, insert_category, insert_product};
use crate::model::{Category, Product};
use crate::query::{get_categories, get_product_by_id, get_products_by_category};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect(".env dosyasının olduğundan emin olalım");
    // sqlx environment dosyasında DATABASE_URL şeklinde bir key olmasını bekler.
    let conn = std::env::var("DATABASE_URL").expect("connection bilgisinin olduğundan emin olalım");
    // veri tabanı bağlantısı hazırlamanın maliyeti yüksek olduğunda pooling özelliğini aktifleştirdik
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&conn)
        .await
        .expect("Veri tabanına bağlanılamadı");

    let category = Category {
        id: 0,
        title: "Telefon".to_string(),
    };
    let inserted = insert_category(&pool, category).await;
    println!("{} isimli kategori eklendi", inserted);
    let categories = get_categories(&pool).await;
    println!("Kategoriler ({})", categories.len());
    for c in categories {
        println!("{}", c);
    }

    let p = Product {
        id: 0,
        title: "MCTS 70-528 .Net Training Kit".to_string(),
        category_id: 1,
        unit_price: 25.44,
    };
    let inserted = insert_product(&pool, p).await;
    println!("'{}' , veri tabanına eklendi.", inserted);

    let inserted = get_product_by_id(&pool, inserted.id).await;
    println!("{}", inserted);

    println!("Kitap kategorisindeki ürünler");
    let products = get_products_by_category(&pool, 1).await;
    for p in products {
        println!("{}", p);
        let discounted = apply_discount(&pool, p.id, 2.5).await;
        println!("%2.5 indirimli yeni fiyat -> {} lira", discounted.unit_price)
    }
}
