use std::{
    fs,
    fs::File,
    io::Read,
};
use serde_json::{
    json,
    Map,
    value::Value,
};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).expect("[ERROR] Can not open file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect(r#"["ERROR"] Can not read file to string."#);

    Map::new()
}
