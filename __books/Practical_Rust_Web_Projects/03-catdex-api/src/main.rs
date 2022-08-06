use actix_web::{App, HttpServer};
use actix_files::{Files};

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

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
                    Files::new("/", "static")
                )
        }
    )
        .bind("127.0.0.1:8090")?
        .run()
        .await
}
