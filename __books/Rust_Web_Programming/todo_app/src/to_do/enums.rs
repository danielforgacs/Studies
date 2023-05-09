use std::fmt::{Display, Formatter, Result};

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

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Self::Done => write!(f, "Done"),
            Self::Pending => write!(f, "Pending"),
        }
    }
}
