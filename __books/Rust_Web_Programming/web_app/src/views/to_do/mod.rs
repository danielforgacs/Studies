mod create;
mod get;
mod edit;
mod delete;

use actix_web::web::{ServiceConfig, get, post, scope};

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
        .route("get", get().to(get::get))
            .route("create/{title}", post().to(create::create))
            .route("edit", post().to(edit::edit))
            .route("delete", post().to(delete::delete))
    );
}
