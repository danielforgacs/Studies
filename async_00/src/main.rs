use futures::executor::block_on;
use futures::join;
use futures::future::join_all;
use std::{thread, time};

fn main() {
    let now = time::Instant::now();
    let futures = async {
        join!(do_something(1), do_something(2))
    };
    let results = block_on(futures);
    println!("elapsed time:: {:?}", now.elapsed());
    println!("outcome: {:?}", results);
}

async fn do_something(num: i8) -> i8 {
    println!("number {} is running.", num);
    let two_secs = std::time::Duration::new(2, 0);
    thread::sleep(two_secs);
    2
}
