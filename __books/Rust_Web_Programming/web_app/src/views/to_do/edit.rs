use actix_web::{web, HttpResponse};

use crate::state::read_file;
use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::processes::process_input;
use crate::jwt::JWToken;

pub async fn edit(to_do_item: web::Json<ToDoItem>, token: JWToken) -> HttpResponse {
    println!("::extracted JWToken: {:?}", token);
    let state = read_file("./state.json");
    let title = to_do_item.title.clone();
    dbg!(&title);
    let status = match state.get(&title) {
        Some(result) => TaskStatus::from(result.to_string().replace('\"', "")),
        None => return HttpResponse::NotFound().json(
            format!("Task: {} not in state.", title)
        )
    };
    let existing_item = to_do_factory(&title, status.clone());
    if status.to_string() == to_do_item.status {
        return HttpResponse::Ok().json(ToDoItems::get_state());
    }
    process_input(existing_item, "edit".to_owned(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state());
}
