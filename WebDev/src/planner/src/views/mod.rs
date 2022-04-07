use actix_web::web;
use std::env;

mod auth;
mod path;
mod work_item;

pub fn views_factory(app: &mut web::ServiceConfig) {
    // authentication ve work_item view'ları için kullanılan factory nesneleri
    // Actix App nesnesi ile ilişkilendirilir.
    auth::auth_factory(app);
    work_item::work_item_factory(app);
}
