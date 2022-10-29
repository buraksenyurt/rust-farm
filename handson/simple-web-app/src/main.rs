/*
   Kaynağım: https://woile.dev/posts/web-app-with-template-in-rust/

   Axum: Rust tarafındaki popüler Web framework'lerden biri
   Serde: Veri yapılarını özellikle JSON formatında serileştirmede kullanılıyor.
   Tokio: Asenkron çalışma zamanı sunan bir network uzmanı.
   Minijinja: Rust için yazılmış bir template engine.

   Örnekteki toml içeriği aşağıdaki komut satırı ile eklenmiştir.

   cargo add axum tokio -F tokio/full serde -F serde/derive minijinja -F minijinja/builtins

   Test için sunucuyu başlattıktan sonra http://localhost:5001/ adresine gidilebilir
   ve ayrıca http://localhost:5001/book gibi template engine için deneme yapılabilir.

*/

mod category;
mod product;
mod repository;
mod template;

use crate::repository::get_category;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    // Router tanımlamalarının yapıldığı satır
    let app = Router::new()
        .route("/", get(index))
        .route("/:category_name", get(get_category));

    // Web sunucusunu başlatan kısım
    axum::Server::bind(&"0.0.0.0:5001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>I Hate Hello World</h1>")
}
