mod path;
mod auth;

use actix_web::web;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
}
