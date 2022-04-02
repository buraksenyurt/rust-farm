use actix_web::{App, HttpServer};

mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Standart olarak belli bir ip:port üstünden web sunucusunu başlatıyoruz.
    HttpServer::new(|| {
        App::new()
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
