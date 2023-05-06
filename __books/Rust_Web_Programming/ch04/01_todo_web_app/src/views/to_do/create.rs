use crate::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::PERSISTENCE_FILE_NAME;
use actix_web::{HttpRequest, HttpResponse};
use serde_json::{Map, Value};
use actix_web::web::Json;
use crate::json_serialization::to_do_items::ToDoItem;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let mut state: Map<String, Value> = read_file(PERSISTENCE_FILE_NAME);
    let title = req
        .match_info()
        .get("title")
        .expect("Can not get title from request.")
        .to_string();
    let title_ref = title.clone();
    let item = to_do_factory(&String::from("pending"), &title).expect("Can not create item.");
    process_input(item, String::from("create"), &mut state);
    HttpResponse::Ok().json(ToDoItem { title: "alksdfhj".to_string(), status: "387653865".to_string() })
}
