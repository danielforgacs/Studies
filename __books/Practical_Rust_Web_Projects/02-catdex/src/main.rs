pub mod models;
pub mod schema;

use self::models::*;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2;

use actix_files::{Files};
use actix_web::{App, HttpServer, HttpResponse};
use actix_web::web;
use serde_json::json;
use handlebars::Handlebars;
use dotenv;

async fn index(handlebars: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Catdex",
        "cats": [
            {
                "name": "British short hair",
                "image_path": "/static/image/british-short-hair.jpg",
            },
            {
                "name": "Persian",
                "image_path": "/static/image/persian.jpg",
            },
            {
                "name": "ragdoll",
                "image_path": "/static/image/ragdoll.jpg",
            },
        ]
    });
    let body = handlebars.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./static/").unwrap();
    let handlebars_webdata = web::Data::new(handlebars);

    // setting up the db connection pool
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("Set the DATABASE_URL env var.");
            return Ok(());
        }
    };
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool");

    println!("http://localhost:8090");

    HttpServer::new(move || {
        App::new()
        .app_data(handlebars_webdata.clone())
        .data(pool.clone())
        .service(
            Files::new("/static", "static")
            // This lists (servs) files in the static folder
            // in the /static path for debugging
            .show_files_listing()
        )
        .route("/", web::get().to(index))
    })
    .bind("localhost:8090")?
    .run()
    .await
}
