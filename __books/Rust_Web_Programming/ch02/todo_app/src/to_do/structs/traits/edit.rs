use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use crate::write_to_file;
use crate::PERSISTENCE_FILE_NAME;

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        println!("'{}' is being set to done.", title);
        state.insert(title.to_string(), json!("done".to_string()));
        write_to_file(PERSISTENCE_FILE_NAME, state);
    }

    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        println!("'{}' is being set to pending.", title);
        state.insert(title.to_string(), json!("pending".to_string()));
        write_to_file(PERSISTENCE_FILE_NAME, state);
    }
}
