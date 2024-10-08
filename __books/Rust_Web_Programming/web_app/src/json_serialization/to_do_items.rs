use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use crate::to_do::enums::TaskStatus;
use serde::Serialize;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use diesel::prelude::*;
use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_count: i8,
    pub done_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_items = vec![];
        let mut done_items = vec![];

        for item in input_items {
            match item {
                ItemTypes::Pending(item) => pending_items.push(item.super_struct),
                ItemTypes::Done(item) => done_items.push(item.super_struct),
            };
        }
        let pending_count = pending_items.len() as i8;
        let done_count = done_items.len() as i8;
        Self {
            pending_items,
            done_items,
            pending_count,
            done_count,
        }
    }

    pub fn get_state_w_file() -> Self {
        // Old way with file based db.
        let state = read_file("./state.json");
        let mut array_buf = Vec::new();
        for (title, value) in state {
            let status = TaskStatus::from(value.to_string().replace('\"', ""));
            let item = to_do_factory(title.as_ref(), status);
            array_buf.push(item);
        }
        Self::new(array_buf)
    }

    pub fn get_state() -> Self {
        let mut connection = establish_connection();
        let mut array_buffer = Vec::new();
        let items = to_do::table
            .order(to_do::columns::id.asc())
            .load::<Item>(&mut connection).unwrap();
        for item in items {
            let status = TaskStatus::from(item.status.to_string());
            let item = to_do_factory(&item.title, status);
            array_buffer.push(item);
        }
        Self::new(array_buffer)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string_pretty(&self).expect("CAN NOT PRETTU SERIALISE ToDoItems");
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
    }
}
