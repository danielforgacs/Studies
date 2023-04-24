use actix_web::{HttpRequest, HttpResponse};
use actix_web::{web, Responder};
use serde_json::{Value, Map};
use std::future::Ready;
use actix_web::http::Error;
use std::future::ready;
use crate::state::read_file;
use crate::PERSISTENCE_FILE_NAME;
use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn get() -> ToDoItems {
    let state = read_file(PERSISTENCE_FILE_NAME);
    let mut array_buffer = Vec::new();
    for (key, value) in state {
        let item_type = String::from(value.as_str().expect("Get() state value error"));
        let item = to_do_factory(&item_type, &key).expect("get() - todo factory error.");
        array_buffer.push(item);

    }
    ToDoItems::new(array_buffer)
}

impl Responder for ToDoItems {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).expect("Resnponder trait:: can't serialize");
        ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))

    }
}
// impl Responder for ToDoItems {
//     type Body: MessageBody + 'static;

//     // Required method
//     fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
//         HttpResponse::build(status)
//     }

//     // // Provided method
//     // fn customize(self) -> CustomizeResponder<Self>
//     //    where Self: Sized { ... }
// }
