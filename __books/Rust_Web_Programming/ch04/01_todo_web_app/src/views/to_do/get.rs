use actix_web::{web, Responder};
use serde_json::{Value, Map};
use crate::state::read_file;
use crate::PERSISTENCE_FILE_NAME;
use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn get() -> impl Responder {
    let state = read_file(PERSISTENCE_FILE_NAME);
    let mut array_buffer = Vec::new();
    for (key, value) in state {
        let item_type = String::from(value.as_str().expect("Get() state value error"));
        let item = to_do_factory(&item_type, &key).expect("get() - todo factory error.");
        array_buffer.push(item);

    }
    let return_package = ToDoItems::new(array_buffer);
    web::Json(return_package)
}
