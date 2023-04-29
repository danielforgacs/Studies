use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::PERSISTENCE_FILE_NAME;
use crate::to_do::to_do_factory;

pub fn return_state() -> ToDoItems {
    let state = read_file(PERSISTENCE_FILE_NAME);
    let mut array_buffer = Vec::new();
    for (key, value) in state {
        let item_type = String::from(value.as_str().expect("Get() state value error"));
        let item = to_do_factory(&item_type, &key).expect("get() - todo factory error.");
        array_buffer.push(item);

    }
    ToDoItems::new(array_buffer)
}
