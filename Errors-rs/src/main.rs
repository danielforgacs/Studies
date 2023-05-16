use std::io::{Error, Result, ErrorKind};

fn main() {
    action_one().and_then(action_two).and_then(action_three).ok();
}

fn action_one() -> Result<u8> {
    println!("action_one");
    let problem: Result<()> = Err(Error::new(ErrorKind::Other, "Error ONE!"));
    problem.map_err(|e| {println!("map_err ONE: {}", e)}).ok();
    Ok(1)
}

fn action_two(_n: u8) -> Result<u8> {
    println!("action_two");
    let problem: Result<()> = Err(Error::new(ErrorKind::Other, "Error TWO!"));
    problem.unwrap_or_else(|e| println!("unwrap_or_else TWO: {}", e));
    Ok(2)
}

fn action_three(_n: u8) -> Result<()> {
    println!("action_three");
    let problem: Result<()> = Err(Error::new(ErrorKind::Other, "Error THREE!"));
    problem.map_err(|e| {println!("map_err THREE: {}", e)}).ok();
    Ok(())
}
