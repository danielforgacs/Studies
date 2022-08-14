#[macro_use]
extern crate diesel;
mod schema;

use diesel::prelude::*;
use diesel::{Queryable, PgConnection};
use diesel::r2d2::{Pool, ConnectionManager};
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    let pool = Pool::builder().build(manager)?;
    let db_conn = pool.get()?;
    let all_items = items_table
        .load::<model::StructItem>(&db_conn);
    dbg!(&all_items);
    Ok(())
}
