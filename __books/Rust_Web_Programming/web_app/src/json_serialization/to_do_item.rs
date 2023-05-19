use serde::Deserialize;

#[derive(Deserialize)]
pub struct ToDoItem {
    title: String,
    status: String,
}
