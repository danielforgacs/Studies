use actix_web::{web, Responder};
use serde_json::{Value, Map};
use crate::state::read_file;
use crate::PERSISTENCE_FILE_NAME;

async fn get() -> impl Responder {
    let state = read_file(PERSISTENCE_FILE_NAME);
    web::Json(state)
}
