mod login;
mod logout;

use actix_web::web;
use super::path::Path;

pub fn auth_factory(app: &mut web::ServiceConfig) {
    let base_path = Path { prefix: String::from("/auth") };

    app
    .route(&base_path.define(String::from("/login")), web::get().to(login::login))
    .route(&base_path.define(String::from("/logout")), web::get().to(logout::logout));
}
