use super::utils::return_state;
use crate::json_serialization::to_do_items::ToDoItem;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use crate::PERSISTENCE_FILE_NAME;
use actix_web::{web, HttpResponse};

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    // Getting the title and the status we want to edit.
    let mut state = read_file(PERSISTENCE_FILE_NAME);
    let title_reference = &to_do_item.title.clone();
    let title = to_do_item.title.clone();

    // If the todo item is not found it's an error.
    let status = match &state.get(title_reference) {
        Some(result) => result.to_string().replace('\"', ""),
        None => return HttpResponse::NotFound().json(format!("{} not in state", title_reference)),
    };

    // if the todo item is found
    // and it's status is the same as what we want to edit
    // return the state as ok.
    if status == to_do_item.status {
        return HttpResponse::Ok().json(return_state());
    }

    // if the todo item is found
    // but it's status is different than what we want
    // do a todo factory process input.
    match to_do_factory(&status, &title) {
        Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("edit"), &mut state),
    }

    HttpResponse::Ok().json(return_state())
}
