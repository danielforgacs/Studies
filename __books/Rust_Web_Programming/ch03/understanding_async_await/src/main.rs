use std::time::Duration;
use std::thread::sleep;

async fn do_something(number: i8) -> i8 {
    println!("number {} is running.", number);
    let two_secs = Duration::new(2, 0);
    sleep(two_secs);
    return 2
}

fn main() {
}
