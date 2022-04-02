use actix_web::web;

mod book;
mod path;

// Bir veya daha fazla view'un klasör tabanlı inşa edileceği düşünülürse,
// orkestrasyonu sağlamak için views_factory fonksiyonundan yararlanabiliriz.
pub fn views_factory(app: &mut web::ServiceConfig) {
    book::book_factory(app);
}
