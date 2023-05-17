mod to_do;
mod state;
mod processes;

use to_do::to_do_factory;
use to_do::enums::TaskStatus;
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
        let status = match state.get(title) {
            Some(result) => result.to_string().replace('\n', ""),
            None => "pending".to_string(),
        };
        let item = to_do_factory(title, status.into());
        processes::process_input(item, command.into(), &mut state);
    }
}
