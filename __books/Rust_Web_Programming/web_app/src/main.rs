/*
>>> mod to_do;
>>> mod state;
mod processes;

use to_do::to_do_factory;
>>> use to_do::enums::TaskStatus;
use to_do::ItemTypes;
use to_do::traits::get::Get;
use to_do::traits::delete::Delete;
use to_do::traits::edit::Edit;
use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        let command: &String = &args[1];
        let title: &String = &args[2];
        let mut state = read_file("./state.json");
        // This gets the status of the todo item
        // if it exists in the state. If it
        // doesn't, the status defaults to "pending"
        let status = match state.get(title) {
            Some(result) => result.to_string().replace('\n', ""),
            None => "pending".to_string(),
        };
        let item = to_do_factory(title, status.into());
        processes::process_input(item, command.into(), &mut state);
    }
}
*/

mod to_do;
mod state;
mod views;
mod processes;

use to_do::enums::TaskStatus;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( ||
        App::new()
            .configure(views::views_factory)
    )
    .bind(("0.0.0.0", 8080)).map_err(|err| {
        println!("Error binding: {}", err);
        err
    })?
    .run()
    .await
}
