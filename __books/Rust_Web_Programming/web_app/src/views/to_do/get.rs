use actix_web::Responder;
use crate::json_serialization::to_do_items::TodoItems;

// pub async fn get() -> impl Responder {
//     let state: Map<String, Value> = read_file("./state.json");
//     let mut array_buffer = Vec::new();
//     for (key, value) in state {
//         let status = TaskStatus::from(value.to_string().replace('\"', ""));
//         let item = to_do_factory(&key, status);
//         array_buffer.push(item);
//     }
//     Json(TodoItems::new(array_buffer))
// }

pub async fn get() -> impl Responder {
    TodoItems::get_state()
}
