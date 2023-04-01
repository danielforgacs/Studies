mod to_do;
mod state;

use to_do::{
    ItemTypes,
    to_do_factory,
    structs::traits::create::Create,
};

fn main() {
    let to_do_item = to_do_factory("pending", "pending item");
    match to_do_item.expect("could not create todo item.") {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!("item: \"{}\" is done.", item.super_struct.title),
    }
}
