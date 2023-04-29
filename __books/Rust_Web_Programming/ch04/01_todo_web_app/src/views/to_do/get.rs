use super::utils::return_state;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::PERSISTENCE_FILE_NAME;
use actix_web::{web, Responder};
use serde_json::{Map, Value};

pub async fn get() -> ToDoItems {
    return_state()
}
