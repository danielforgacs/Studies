#[macro_use]
extern crate diesel;

mod schema;

use dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
// use schema;

#[derive(Queryable, Debug)]
struct Stuff {
    id: i32,
    cool: String,
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    use crate::schema::stuff::dsl::*;

    let db_conn = establish_connection();
    let results = stuff
        .limit(3)
        .load::<Stuff>(&db_conn)
        .map_err(|e| dbg!("error: {}", e));
    dbg!(results);
    Ok(())
}
