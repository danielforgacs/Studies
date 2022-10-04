#[derive(Debug, Clone)]
enum FileOpenError{
    Error_A,
}

fn main() {
    let db_file = open_file("db_file_name.txt").unwrap();
}

fn open_file(db_file_name: &str) -> Result<std::fs::File, FileOpenError> {
    Err(FileOpenError::Error_A)
}
