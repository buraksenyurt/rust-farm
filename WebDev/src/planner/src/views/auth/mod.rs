use super::path::Path;
use actix_web::web;
use log::info;

mod login;
mod logout;

pub fn auth_factory(app: &mut web::ServiceConfig) {
    info!("Authentication factory");
    let base_path: Path = Path {
        prefix: String::from("/auth"),
    };

    // auth/login olarak gelen talepleri login modülündeki login fonksiyonuna yönlendiriyoruz
    // http get
    let app = app.route(
        &base_path.define(String::from("/login")),
        web::get().to(login::login),
    );

    // Logout işlemi içinde auth/logout için gelen HTTP Get talebini
    // logout modülündeki logout sayfasına yönlendiriyoruz.
    app.route(
        &base_path.define(String::from("/logout")),
        web::get().to(logout::logout),
    );
}
