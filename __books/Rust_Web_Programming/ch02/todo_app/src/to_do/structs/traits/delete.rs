use serde_json::Map;
use serde_json::value::Value;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        println!("'{}' is being deleted.", title);
        state.remove(title);
        println!("Deleted: '{}'.", title);
    }
}
