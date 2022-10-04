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
}

fn open_file(db_file_name: &str) -> Result<std::fs::File, FileOpenError> {
    std::fs::File::open(db_file_name)?;
    Err(FileOpenError::ErrorA("EOF".to_string()))
}
