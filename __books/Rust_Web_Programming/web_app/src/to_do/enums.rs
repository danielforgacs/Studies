use std::fmt::{Display, Formatter, Result};
use serde::{Serialize};

#[derive(Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Self::DONE => write!(f, "DONE"),
            Self::PENDING => write!(f, "PENDING"),
        }
    }
}

impl From<String> for TaskStatus {
    fn from(value: String) -> Self {
        let value_raw = value.clone();
        match value.as_str() {
            "PENDING" => Self::PENDING,
            "DONE" => Self::DONE,
            _ => {
                panic!("UNSUPPORTED TaskStatus STRING: >>>{:?}<<<, {:?}", value_raw, value_raw.chars());
            },
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
