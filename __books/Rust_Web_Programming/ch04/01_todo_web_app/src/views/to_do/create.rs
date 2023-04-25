use actix_web::HttpRequest;
use serde_json::{Map, Value};
use crate::state::read_file;
use crate::PERSISTENCE_FILE_NAME;
use crate::to_do::to_do_factory;
use crate::process_input;

pub async fn create(req: HttpRequest) -> String {
    let mut state: Map<String, Value> = read_file(PERSISTENCE_FILE_NAME);
    let title = req.match_info().get("title").expect("Can not get title from request.").to_string();
    let title_ref = title.clone();
    let item = to_do_factory(&String::from("pending"), &title).expect("Can not create item.");
    process_input(item, String::from("create"), &mut state);
    format!("Created item: {}", title_ref)
}
