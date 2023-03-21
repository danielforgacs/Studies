pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(title: &str, status: &str) -> Self {
        Self {
            title: title.to_string(),
            status: status.to_string(),
        }
    }
}
