use std::env::args;

fn main() {
    let result = args()
    .map(|mut i| {i.push(' '); i})
    .collect::<Vec<String>>();
    println!("{:?}", result);
}
