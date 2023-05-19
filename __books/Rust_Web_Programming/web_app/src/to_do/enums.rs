use std::fmt::{Display, Formatter, Result};
use serde::{Serialize};

pub enum TaskStatus {
    DONE,
    PENDING,
}

// impl TaskStatus {
//     pub fn stringify(&self) -> String {
//         match &self {
//             &Self::DONE => {"DONE".to_string()},
//             &Self::PENDING => {"PENDING".to_string()}
//         }
//     }
// }

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Self::DONE => write!(f, "Done"),
            Self::PENDING => write!(f, "Pending"),
        }
    }
}

impl From<String> for TaskStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "pending" => Self::PENDING,
            _ => Self::DONE,
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        Ok(serializer.serialize_str(self.to_string().as_str())?)
    }
}
