use serde_json::Map;
use serde_json::Value;

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        println!("{} is being fetched", title);
        let item = state.get(title);
        match item {
            Some(result) => {
                println!("found.");
                println!("item title: {}", title);
                println!("item status: {}", result);
            },
            None => println!("not found."),
        };
    }
}
