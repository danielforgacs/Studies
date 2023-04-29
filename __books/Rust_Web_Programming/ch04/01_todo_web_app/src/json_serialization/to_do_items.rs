use actix_web::Responder;
use actix_web::{HttpRequest, HttpResponse};
use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub donr_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_arraw_buffer = Vec::new();
        let mut done_arraw_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_arraw_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_arraw_buffer.push(packed.super_struct),
            }
        }

        let pending_count = pending_arraw_buffer.len() as i8;
        let done_count = done_arraw_buffer.len() as i8;

        ToDoItems {
            pending_items: pending_arraw_buffer,
            done_items: done_arraw_buffer,
            pending_item_count: pending_count,
            donr_item_count: done_count,
        }
    }
}

impl Responder for ToDoItems {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let json_body = serde_json::to_string_pretty(&self).expect("Can't serialize ToDoItems.");
        HttpResponse::Ok()
        .content_type("application/json")
        .body(json_body)
    }
}
