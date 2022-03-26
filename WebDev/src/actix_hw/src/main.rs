use actix_web::{web, App, HttpRequest, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("Sunucu başlatıldı");
        let app = App::new()
            .route("/", web::get().to(wellcome))
            .route("/{username}", web::get().to(wellcome));
        app
    })
    .bind("0.0.0.0:8000")?
    .workers(2)
    .run()
    .await
}

async fn wellcome(req: HttpRequest) -> impl Responder {
    println!("{:#?}", req);
    let username = req.match_info().get("username").unwrap_or("Yabancı");
    format!("Merhaba {}", username)
}
