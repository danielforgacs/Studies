use serde_json::{json, value::Value, Map};

use crate::write_to_file;
use crate::PERSISTENCE_FILE_NAME;

pub trait Create {
    fn create(&self, title: &str, status: &String, state: &mut Map<String, Value>) {
        println!("'{}' is being created.", title);
        state.insert(title.to_string(), json!(status));
        write_to_file(PERSISTENCE_FILE_NAME, state);
        println!("done creating: '{}'.", title);
    }
}
