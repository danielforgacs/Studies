use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use crate::to_do::enums::TaskStatus;
use serde::Serialize;

#[derive(Serialize)]
pub struct TodoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_count: i8,
    pub done_count: i8,
}

// impl Serialize for TodoItems {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {

//     }
// }

impl TodoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_items = vec![];
        let mut done_items = vec![];

        for item in input_items {
            match item {
                ItemTypes::Pending(item) => pending_items.push(item.super_struct),
                ItemTypes::Done(item) => done_items.push(item.super_struct),
            };
        }
        let pending_count = pending_items.len() as i8;
        let done_count = done_items.len() as i8;
        Self {
            pending_items,
            done_items,
            pending_count,
            done_count,
        }
    }
}
