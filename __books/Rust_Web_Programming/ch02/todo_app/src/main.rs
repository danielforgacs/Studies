mod to_do;

use to_do::{
    ItemTypes,
    to_do_factory,
};

fn main() {
    let to_do_item = to_do_factory("pending", "make");
    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => println!("item: \"{}\" is pending.", item.super_struct.title),
        ItemTypes::Done(item) => println!("item: \"{}\" is done.", item.super_struct.title),
    }
}
