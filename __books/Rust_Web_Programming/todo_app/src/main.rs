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
        let status: &String = &args[1];
        let title: &String = &args[2];
        let item_type = to_do_factory(title, TaskStatus::PENDING);
        let item = match item_type {
            ItemTypes::Pending(item) => item,
            _ => panic!("NOT PROCESSED YET."),
        };
        let mut state: Map<String, Value> = read_file("./state.json");
        processes::process_pending(item, "create".to_string(), &mut state);
        // println!("Before operation: {:?}", state);
        // state.insert(title.to_string(), json!(status));
        // println!("After operation: {:?}", state);
        // write_to_file("./state.json", &mut state);
    }
}
