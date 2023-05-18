use actix_web::HttpRequest;
use serde_json::{Map, Value};
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::to_do::enums::TaskStatus;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json");
    let title = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(&title, TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);
    format!("{} created", title)
}
