use std::path::{Path, PathBuf};

fn main() {
    {
        let file_name = "__test_file_name.txt".to_string();
        let file_path = Path::new(&file_name);
        dbg!(&file_path);
        let abs_path = file_path.canonicalize();
        dbg!(&abs_path);
    }

    {
        let file_name = "__test_file_name.txt".to_string();
        let mut file_path = PathBuf::new();
        file_path.push(file_name);
        dbg!(&file_path);
        let abs_path = file_path.canonicalize();
        dbg!(&abs_path);
    }
}
