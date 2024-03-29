mod to_do;
mod state;
mod processes;

use to_do::{
    ItemTypes,
    to_do_factory,
};
use state::{write_to_file, read_file};
use processes::process_input;

pub const PERSISTENCE_FILE_NAME: &str = "./state.json";

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let command = &args.get(1).expect("[ERROR] Add a command argument.");
    let title = args.get(2).expect("[ERROR] Add a title argument.");
    let mut state = read_file(PERSISTENCE_FILE_NAME);
    let status: String;
    match state.get(title) {
        Some(result) => {status = result.to_string().replace('\"', "")},
        None => {status = "pending".to_string()},
    }
    let item = to_do_factory(&status, title).expect(&status);
    println!("factory item: {:?}", &item);
    process_input(item, command.to_string(), &mut state);
}
