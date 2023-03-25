mod to_do;

use to_do::structs::{done::Done, pending::Pending};

fn main() {
    let done = Done::new("shopping todo");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);

    let pending = Pending::new("pending todo");
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status);
}
