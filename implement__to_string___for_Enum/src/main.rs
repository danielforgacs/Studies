enum Stuff {
    Alpha,
    Beta,
}

impl ToString for Stuff {
    fn to_string(&self) -> String {
        match self {
            Stuff::Alpha => { return "alpha".to_string(); }
            Stuff::Beta => { return "Beta, not alpha".to_string(); }
        }
    }
}

fn main() {
    let thing = Stuff::Alpha;
    println!("thing is: {}", thing.to_string());
    let thing = Stuff::Beta;
    println!("thing is: {}", thing.to_string());
}
