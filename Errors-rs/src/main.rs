use std::io::{Error, Result, ErrorKind};

fn main() {
    action_one().and_then(action_two).and_then(action_three).ok();
}

fn action_one() -> Result<u8> {
    Ok(1)
}

fn action_two(_n: u8) -> Result<u8> {
    Ok(2)
}

fn action_three(_n: u8) -> Result<()> {
    Err(Error::new(ErrorKind::Other, "Shit this!"))
}
