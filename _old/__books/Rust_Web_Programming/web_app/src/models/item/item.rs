use diesel::prelude::*;
use crate::schema::to_do;
use chrono::{NaiveDateTime, Utc};

#[derive(Queryable, Identifiable)]
#[table_name="to_do"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
}
