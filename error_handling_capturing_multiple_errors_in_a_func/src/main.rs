/*

basic example:
https://stackoverflow.com/questions/71812362/rust-error-handling-capturing-multiple-errors

*/

use std::io::prelude::*;

#[derive(Debug, Clone)]
enum FileOpenError{
    ErrorA(String),
}

impl From<std::io::Error> for FileOpenError {
    fn from(err: std::io::Error) -> Self {
        Self::ErrorA(format!("[ERROR STUFF::{}::{}]", err.kind(), err.to_string()))
    }
}

fn main() {
    let _db_file = match open_file("db_file_name.txt") {
        Err(FileOpenError::ErrorA(msg)) => println!("{}", msg),
        Ok(_) => (),
    };
    match another_file_error() {
        Err(FileOpenError::ErrorA(msg)) => println!("{}", msg),
        Ok(_) => (),
    };
}

fn open_file(db_file_name: &str) -> Result<std::fs::File, FileOpenError> {
    std::fs::File::open(db_file_name)?;
    Err(FileOpenError::ErrorA("EOF".to_string()))
}

fn another_file_error() -> Result<(), FileOpenError> {
    // Set the permissions to 0 before running this,
    // so the error show up!
    // $ chmod 0 __delete.txt
    let mut file = std::fs::File::create("__delete.txr")?;
    let buf = String::new();
    file.write_all(&buf.as_bytes()).unwrap();
    Ok(())
}
