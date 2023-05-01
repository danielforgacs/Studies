use super::utils::return_state;
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn get() -> ToDoItems {
    return_state()
}
