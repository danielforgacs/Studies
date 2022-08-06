mod schema;

#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, HttpResponse, Responder};
use actix_web::web;
use actix_files::{Files, NamedFile};
use serde::{Serialize};
use diesel::prelude::*;
use crate::schema::cats::dsl::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[derive(Queryable, Serialize, Debug)]
struct Cats {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

async fn alive() -> impl Responder {
    "alive"
}

async fn api_cats() -> HttpResponse {
    let mut conn = establish_connection();
    let query = cats
        .load::<Cats>(&mut conn)
        .expect("Can't query cats.");
    HttpResponse::Ok().json(query)
}

async fn index() -> Result<NamedFile, std::io::Error> {
    Ok(NamedFile::open("./static/index.html")?)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        move || {
            App::new()
                .service(
                    Files::new("/static", "static")
                )
                .service(
                    web::scope("/api")
                        .route("/cats", web::get().to(api_cats))
                )
                .route("/alive", web::get().to(alive))
                .route("/", web::get().to(index))
        }
    )
        .bind("127.0.0.1:8090")?
        .run()
        .await
}
