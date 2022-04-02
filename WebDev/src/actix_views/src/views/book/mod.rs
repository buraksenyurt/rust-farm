use crate::views::path::Path;
use actix_web::web;

mod create;
mod delete;
mod get;
mod update;

/*
   Factory işlevi gören fonksiyon actix'in ServiceConfig veri yapısını kullanarak,
   uygulama için gerekli yönlendirme tanımlarını ekliyor.
   Buna göre http üstünden gelen yönlendirmeler şöyle eşleşecek;

   GET /book/get için get modülündeki get fonksiyonu
   POST /book/create için create modülündeki create fonksiyonu
   DELETE /book/delete için delete modülündeki delete fonksiyonu
   UPDATE /book/update için update modülündeki update fonksiyonu
*/
pub fn book_factory(app: &mut web::ServiceConfig) {
    let base_path = Path {
        prefix: String::from("/book"),
    };
    // Route tanımlamalarında ikinci parametre ile gelen web talebi metodu neyse
    // (get,post,put,delete) ona uygun bir modül fonksiyonuna yönlendirme yapılmakta.
    app.route(
        &base_path.define(String::from("/get")),
        web::get().to(get::get),
    )
    .route(
        &base_path.define(String::from("/create")),
        web::post().to(create::create),
    )
    .route(
        &base_path.define(String::from("/delete")),
        web::delete().to(delete::delete),
    )
    .route(
        &base_path.define(String::from("/update")),
        web::put().to(update::update),
    );
}
