mod structs;

use structs::{
    done::Done,
    pending::Pending,
};

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    if item_type == "pending" {
        Ok(ItemTypes::Pending(Pending::new(item_title)))
    } else if item_type == "done" {
        Ok(ItemTypes::Done(Done::new(item_title)))
    } else {
        Err("This item type is not accepted.")
    }
}
