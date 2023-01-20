mod model;
mod network;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Başlangıç için kullanılacak bir wellcome sayfası
    let root = warp::path::end().map(|| "Wellcome page");
    // CORS ayarlaması. Herhangi bir kaynaktan gelinebilir
    let routes = root.with(warp::cors().allow_any_origin());
    println!("Sunucu dinlemede. 127.0.0.1:5555");
    // 127.0.0.1:5555 porttan sunucu açılır
    warp::serve(routes).run(([127, 0, 0, 1], 5555)).await;
}
