use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::Done,
        };
        Self { super_struct: base }
    }
}
