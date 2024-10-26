use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Cat {
    id: i32,
    name: String,
    image_path: String,
}
