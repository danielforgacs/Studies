use crate::write_to_file;
use crate::PERSISTENCE_FILE_NAME;
use serde_json::value::Value;
use serde_json::Map;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        println!("'{}' is being deleted.", title);
        state.remove(title);
        write_to_file(PERSISTENCE_FILE_NAME, state);
        println!("Deleted: '{}'.", title);
    }
}
