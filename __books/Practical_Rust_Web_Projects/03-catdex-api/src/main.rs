mod schema;

use crate::schema::cats::dsl::*;

use actix_web::{App, HttpServer};
use actix_files::{Files};

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[derive(Queryable, Debug)]
struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
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
    let mut conn = establish_connection();
    let query = cats
        .load::<Cat>(&mut conn)
        .expect("Can't query cats.");
    dbg!(&query);

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
