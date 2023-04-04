mod to_do;
mod state;

use to_do::{
    ItemTypes,
    to_do_factory,
    structs::traits::create::Create,
};
use state::{write_to_file, read_file};
use serde_json::{json};

pub const PERSISTENCE_FILE_NAME: &str = "./state.json";

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let status = &args.get(1).expect("[ERROR] Add a status argument.");
    let title = &args.get(2).expect("[ERROR] Add a title argument.");
    let mut state = read_file(PERSISTENCE_FILE_NAME);
    println!("state: {:?}", state);
    state.insert(title.to_string(), json!(status));
    write_to_file(PERSISTENCE_FILE_NAME, &state);
}
