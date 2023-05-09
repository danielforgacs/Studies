pub enum TaskStatus {
    Done,
    Pending,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::Done => {"DONE".to_string()},
            &Self::Pending => {"PENDING".to_string()}
        }
    }
}
