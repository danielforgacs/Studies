use serde::{Serialize};
use super::schema::cats;

#[derive(Queryable, Serialize)]
struct Cat {
    id: i32,
    name: String,
    image_name: String,
}
