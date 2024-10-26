use std::{thread, time};
use std::thread::JoinHandle;

fn do_something(num: i8) -> i8 {
    println!("num: {} is running", num);
    let two_secs =  time::Duration::new(2, 0);
    thread::sleep(two_secs);
    num * 2
}

fn main() {
    // main_01();
    // println!("=== threaded version =================");
    // main_w_threads();
    println!("=== threaded Vec =================");
    let now = time::Instant::now();
    vec_from_threads();
    println!(":: {:?}", now.elapsed());
}

fn main_01() {
    let now = time::Instant::now();

    let one = do_something(1);
    let two = do_something(2);
    let three = do_something(3);

    println!("elapsed: {:?}", now.elapsed());
    println!("sum: {}", one + two + three);
}

fn main_w_threads() {
    let now = time::Instant::now();

    let thread_one = thread::spawn(|| do_something(1));
    let thread_two = thread::spawn(|| do_something(2));
    let thread_three = thread::spawn(|| do_something(3));

    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let result_three = thread_three.join();

    println!("elapsed: {:?}", now.elapsed());
    println!("sum: {}", result_one.unwrap() + result_two.unwrap() + result_three.unwrap());
}

fn vec_from_threads() {
    let mut threads = vec![];
    for i in 0..15 {
        threads.push(
            thread::spawn(move || do_something(i))
        )
    }
    let resuslts: Vec<_> = threads
        .into_iter()
        .map(|f| f.join())
        .map(|f| f.unwrap())
        .collect();
    println!("results: {:?}", resuslts);
}
