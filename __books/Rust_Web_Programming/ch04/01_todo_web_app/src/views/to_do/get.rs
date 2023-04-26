use actix_web::{web, Responder};
use serde_json::{Value, Map};
use crate::state::read_file;
use crate::PERSISTENCE_FILE_NAME;
use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_items::ToDoItems;
use super::utils::return_state;

pub async fn get() -> ToDoItems {
    return_state()
}
