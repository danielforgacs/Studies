use std::fs::File;
use std::fs;
use std::io::Read;
use std::io::Write;
use serde_json::Map;
use serde_json::value::Value;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).unwrap_or_else(|_err| {
        let mut file = File::create(file_name).unwrap();
        file.write_all(b"{\n}").unwrap();
        File::open(file_name.to_string()).unwrap()
    });
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = serde_json::to_string_pretty(state).unwrap();
    fs::write(
        file_name.to_string(),
        new_data
    ).expect("Unable to write file");
}
