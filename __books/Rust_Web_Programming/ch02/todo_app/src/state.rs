use std::{
    fs,
    fs::File,
    io::Read,
};
use serde_json::{
    self,
    json,
    Map,
    value::Value,
};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).expect("[ERROR] Can not open file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect(r#"["ERROR"] Can not read file to string."#);
    let json_ = serde_json::from_str::<Value>(&data).expect("Could not deserialize data to json.");
    let state = json_.as_object().expect("Can not convert serde_json 'Value'.").clone();
    state
}

pub fn write_to_file(file_name: &str, state: &Map<String, Value>) {
    // let new_data = json!(state);
    // fs::write(file_name.to_string(), new_data.to_string()).expect("Can not write json data.");
    let new_data = serde_json::to_string_pretty(&state).expect(
        "Could not serialise the data to pretty string."
    );
    fs::write(file_name.to_string(), new_data).expect("Could not write json file.");
}
