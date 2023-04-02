use serde_json::{
    Map,
    Value,
};

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        println!("'{}' is being fetched.", title);
        let item = state.get(title);
        match item {
            Some(result) => {
                println!("Item: {}", title);
                println!("status: {}\n\n", result);
            }
            None => println!("Item: {} was not found.", title),
        }
    }
}
