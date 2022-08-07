mod schema;

#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, HttpResponse, Responder};
use actix_web::web;
use actix_files::{Files, NamedFile};
use serde::{Serialize};
use diesel::prelude::*;
use crate::schema::cat::dsl::*;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;
use r2d2;

#[derive(Queryable, Serialize, Debug)]
struct Cats {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

async fn alive() -> impl Responder {
    "alive"
}

async fn api_cats(pool: web::Data<r2d2::Pool<ConnectionManager::<diesel::PgConnection>>>) -> HttpResponse {
    let mut conn = pool.get().expect("can't get db pool in 'api_cats()'");
    let query = web::block(move || {
        cat.load::<Cats>(&mut conn)
    })
    .await
    .expect("something's wrong");
    HttpResponse::Ok().json(query)
}

async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open_async("./static/index.html").await
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
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<diesel::PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .max_size(25)
        .build(manager)
        .expect("Can't create db connection pool");

    HttpServer::new(
        move || {
            App::new()
                .app_data(pool.clone())
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
