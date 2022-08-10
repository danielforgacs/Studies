#[macro_use]
extern crate diesel;
mod schema;

use diesel::prelude::*;
use diesel::{PgConnection, Queryable};
use schema::items_table::dsl::*;
use dotenv;

mod model {
    use super::*;
    #[derive(Queryable)]
    struct StructItem {
        id: i32,
        item_name_field: String,
    }
}

fn establish_db_connection() -> Result<PgConnection, Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&db_url)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db_conn = establish_db_connection()?;
    // let all_items =
    Ok(())
}
