use actix_web::{HttpRequest, HttpResponse, web};
use serde_json::{Map, Value};
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::to_do::enums::TaskStatus;
use crate::processes::process_input;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::jwt::JWToken;

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JWToken) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let status = match &state.get(&to_do_item.title) {
        Some(result) => TaskStatus::from(result.to_string().replace('\"', "")),
        None => return HttpResponse::NotFound().json(format!("item: \"{}\" not in state.", &to_do_item.title))
    };
    let item = to_do_factory(&to_do_item.title, status);
    process_input(item, "delete".to_string(), &state);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
