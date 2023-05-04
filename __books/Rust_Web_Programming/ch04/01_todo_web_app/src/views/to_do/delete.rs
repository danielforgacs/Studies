use crate::json_serialization::to_do_items::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::to_do::ItemTypes;
use crate::PERSISTENCE_FILE_NAME;
use actix_web::web::Json;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use serde_json::{Map, Value};

// pub async fn delete(req: HttpRequest) -> String {
//     let title = req
//         .match_info()
//         .get("title")
//         .expect("Can not get title from request.")
//         .to_string();
//     let item = to_do_factory(&String::from("pending"), &title).expect("Can not create item.");
//     let mut state: Map<String, Value> = read_file(PERSISTENCE_FILE_NAME);
//     process_input(item, String::from("delete"), &mut state);
//     format!("Deleted item: {}", &title)
// }

pub async fn delete(item: Json<ToDoItem>) -> HttpResponse {
    let mut state: Map<String, Value> = read_file(PERSISTENCE_FILE_NAME);
    // let title_clone = item.into_inner().title;
    // let status_clone = item.into_inner().status;
    let title_clone = item.title.clone();
    let status_clone = item.status.clone();
    match to_do_factory(&item.status, &item.title) {
        Ok(item_type) => process_input(item_type, "delete".to_string(), &mut state),
        Err(error) => {
            return HttpResponse::BadRequest().json(format!(
                "Not acceped: {}: {}. error: {}",
                title_clone, status_clone, error
            ))
        }
    };
    HttpResponse::Ok().json(state)
}
