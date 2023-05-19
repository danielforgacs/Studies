use actix_web::Responder;
use actix_web::web::Json;
use serde_json::{Map, Value};
use crate::state::read_file;

pub async fn get() -> impl Responder {
    let state: Map<String, Value> = read_file("./state.json");
    Json(state)
}
