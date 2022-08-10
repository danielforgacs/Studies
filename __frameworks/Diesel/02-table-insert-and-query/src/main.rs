#[macro_use]
extern crate diesel;
mod schema;

use diesel::prelude::*;
use diesel::{PgConnection, Queryable};
use diesel::r2d2;
use schema::items_table::dsl::*;
use dotenv;

mod model {
    use super::*;
    #[derive(Queryable, Debug)]
    pub struct StructItem {
        id: i32,
        item_name_field: Option<String>,
    }
}

fn establish_db_connection() -> Result<PgConnection, Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&db_url)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let manager = r2d2::ConnectionManager::<PgConnection>::new(&db_url);
    let db_conn = establish_db_connection()?;
    let pool = r2d2::Pool::builder().build(manager)?;
    let all_items = items_table
        .load::<model::StructItem>(&db_conn);
    dbg!(&all_items);
    Ok(())
}
