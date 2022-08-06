mod schema;

use crate::schema::cats::dsl::*;

use actix_web::{App, HttpServer, HttpResponse, Responder};
use actix_web::web;
use actix_files::{Files};
use serde::{Serialize};

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
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
    println!("alive");
    "alive"
}

async fn api_cats() -> HttpResponse {
    println!("api_cats()");
    let mut conn = establish_connection();
    let query = cats
        .load::<Cats>(&mut conn)
        .expect("Can't query cats.");
    dbg!(&query);
    HttpResponse::Ok().json(query)
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
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
                    Files::new("/static", ".")
                )
                .service(
                    web::scope("/api")
                        .route("/cats", web::get().to(api_cats))
                )
                .route("/alive", web::get().to(alive))
        }
    )
        .bind("127.0.0.1:8090")?
        .run()
        .await
}
