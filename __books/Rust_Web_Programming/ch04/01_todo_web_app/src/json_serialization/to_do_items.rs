use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub donr_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_arraw_buffer = Vec::new();
        let mut done_arraw_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_arraw_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_arraw_buffer.push(packed.super_struct),
            }
        }

        let pending_count = pending_arraw_buffer.len() as i8;
        let done_count = done_arraw_buffer.len() as i8;

        ToDoItems {
            pending_items: pending_arraw_buffer,
            done_items: done_arraw_buffer,
            pending_item_count: pending_count,
            donr_item_count: done_count,
        }
    }
}
