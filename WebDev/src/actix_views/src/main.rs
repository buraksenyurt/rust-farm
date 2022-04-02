use actix_web::{App, HttpServer};

mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //Standart olarak belli bir ip:port üstünden web sunucusunu başlatıyoruz.
    // App nesne örneğinin configure fonksiyonuna parametre olarak views_factory fonksiyonunu verdik.
    // views_factory'de bu örnek için book_factory fonksiyonunu kullanmakta.
    // book_factory, /book root adresine gelen talepleri ele alacak fonksiyonlar için gerekli yönlendirmeleri içermekte.
    // Dolayısıyla app nesnesi, gelen taleplerin hangi view ve fonksiyonlara indirilmesi gerektiğini biliyor.
    HttpServer::new(|| {
        println!("Sunucu dinlemede. localhost:8000");
        App::new().configure(views::views_factory)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
