use std::env::args;

fn main() {
    let result = args()
    .map(|mut i| {i.push(' '); i})
    .collect::<Vec<String>>();
    dbg!(&result);
    // println!("{}", result);
}
