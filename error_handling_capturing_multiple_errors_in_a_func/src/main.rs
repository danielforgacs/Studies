#[derive(Debug, Clone)]
enum FileOpenError{
    Error_A(String),
}

impl From<std::io::Error> for FileOpenError {
    fn from(err: std::io::Error) -> Self {
        Self::Error_A("alksdfhj".to_string())
    }
}

fn main() {
    let db_file = match open_file("db_file_name.txt") {
        Err(FileOpenError::Error_A(msg)) => println!("{}", msg),
        Ok(_) => (),
    };
}

fn open_file(db_file_name: &str) -> Result<std::fs::File, FileOpenError> {
    std::fs::File::open(db_file_name)?;
    Err(FileOpenError::Error_A("EOF".to_string()))
}
