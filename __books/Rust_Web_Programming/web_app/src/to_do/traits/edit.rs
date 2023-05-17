use crate::state::write_to_file;
use serde_json::{Map, Value, json};
use crate::TaskStatus;

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        println!("{} is being set to done", title);
        state.insert(title.to_string(), json!(TaskStatus::DONE.to_string()));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to done\n\n", title);
    }

    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        println!("{} is being set to pending", title);
        state.insert(title.to_string(), json!(TaskStatus::PENDING.to_string()));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to pending\n\n", title);
    }
}
