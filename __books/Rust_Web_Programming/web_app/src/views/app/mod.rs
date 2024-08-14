mod items;
mod content_loader;

use actix_web::web::{ServiceConfig, get, post, scope};

pub fn app_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/")
        .route("", get().to(items::items))
    );
}
