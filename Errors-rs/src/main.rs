use std::io::{Error, Result, ErrorKind};

fn main() {
    action_one().and_then(action_two).and_then(action_three).ok();
}

fn action_one() -> Result<u8> {
    println!("action_one");
    let problem: Result<()> = Err(Error::new(ErrorKind::Other, "Error ONE!"));
    problem.map_err(|e| {println!("shit happend ONE: {}", e)}).ok();
    Ok(1)
}

fn action_two(_n: u8) -> Result<u8> {
    println!("action_two");
    let problem: Result<()> = Err(Error::new(ErrorKind::Other, "Error TWO!"));
    problem.map_err(|e| {println!("shit happend TWO: {}", e)}).ok();
    Ok(2)
}

fn action_three(_n: u8) -> Result<()> {
    println!("action_three");
    let problem: Result<()> = Err(Error::new(ErrorKind::Other, "Error THREE!"));
    problem.map_err(|e| {println!("shit happend THREE: {}", e)}).ok();
    Ok(())
}
