mod command;
mod model;
mod query;

use crate::query::get_categories;
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

    let categories = get_categories(pool).await;
    println!("Kategoriler ({})", categories.len());
    for c in categories {
        println!("{}", c);
    }
}
