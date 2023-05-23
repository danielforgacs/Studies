use actix_web::{HttpRequest, HttpResponse};
use serde_json::{Map, Value};
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::to_do::enums::TaskStatus;
use crate::processes::process_input;
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let title = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(&title, TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);
    HttpResponse::Ok()
        .json(ToDoItems::get_state())
}
