/*
   Kaynağım: https://woile.dev/posts/web-app-with-template-in-rust/

   Axum: Rust tarafındaki popüler Web framework'lerden biri
   Serde: Veri yapılarını özellikle JSON formatında serileştirmede kullanılıyor.
   Tokio: Asenkron çalışma zamanı sunan bir network uzmanı.
   Minijinja: Rust için yazılmış bir template engine.

   Örnekteki toml içeriği aşağıdaki komut satırı ile eklenmiştir.

   cargo add axum tokio -F tokio/full serde -F serde/derive minijinja -F minijinja/builtins

*/

use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    // Router tanımlamalarının yapıldığı satır
    let app = Router::new().route("/", get(index));

    // Web sunucusunu başlatan kısım
    axum::Server::bind(&"0.0.0.0:5001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>I Hate Hello World</h1>")
}
