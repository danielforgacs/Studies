mod to_do;
mod state;

use to_do::{
    ItemTypes,
    to_do_factory,
    structs::traits::create::Create,
};
use state::{write_to_file, read_file};
use serde_json::{json};

const PERSISTENCE_FILE_NAME: &str = "./state.json";

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let status = &args[1];
    let title = &args[2];
    let mut state = read_file(PERSISTENCE_FILE_NAME);
    println!("state: {:?}", state);
    state.insert(title.to_string(), json!(status));
    write_to_file(PERSISTENCE_FILE_NAME, &state);
}
