use futures::executor::block_on;
use std::time::{Duration, Instant};
use std::thread::sleep;

async fn do_something(number: i8) -> i8 {
    println!("number {} is running.", number);
    let two_secs = Duration::new(2, 0);
    sleep(two_secs);
    return 2
}

fn main() {
    let now = Instant::now();
    let future_one = do_something(1);
    let outcome = block_on(future_one);
    println!("elapsed time: {:?}", now.elapsed());
    println!("here is the outcome: {}", outcome);
}
