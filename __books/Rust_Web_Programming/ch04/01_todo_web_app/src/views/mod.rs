mod path;
mod auth;
mod to_do;
mod token;

use actix_web::web;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    to_do::item_factory(app);
}
