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
use diesel::r2d2::{ConnectionManager, Pool};
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

async fn api_cats(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    let mut conn = pool.get().expect("Can't get connection pool.");
    let query = web::block(move || {
        cat.load::<Cats>(&mut conn)
    })
    .await
    .unwrap()
    .unwrap();
    // .expect("Query failed.");
    dbg!(&query);
    HttpResponse::Ok().json(query)
    // HttpResponse::Ok().finish()
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
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");
    HttpServer::new(
        move || {
            App::new()
                .app_data(web::Data::new(pool.clone()))
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
