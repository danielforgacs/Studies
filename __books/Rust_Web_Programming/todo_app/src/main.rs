mod to_do;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

fn main() {
    let done = Done::new("shopping");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);
    println!("{}", done.super_struct.status.stringify());
    let landry = Pending::new("landry");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);
    println!("{}", done.super_struct.status.stringify());
}
