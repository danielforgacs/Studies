use actix_web::HttpRequest;
use crate::to_do::to_do_factory;
use crate::processes::process_input;
use crate::PERSISTENCE_FILE_NAME;
use crate::state::read_file;
use serde_json::{Map, Value};

pub async fn delete(req: HttpRequest) -> String {
    let title = req
        .match_info()
        .get("title")
        .expect("Can not get title from request.")
        .to_string();
    let item = to_do_factory(&String::from("pending"), &title).expect("Can not create item.");
    let mut state: Map<String, Value> = read_file(PERSISTENCE_FILE_NAME);
    process_input(item, String::from("delete"), &mut state);
    format!("Deleted item: {}", &title)
}
