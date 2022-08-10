use diesel::prelude::*;
use diesel::PgConnection;
use dotenv;

fn establish_db_connection() -> Result<PgConnection, Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&db_url)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db_conn = establish_db_connection()?;
    Ok(())
}
