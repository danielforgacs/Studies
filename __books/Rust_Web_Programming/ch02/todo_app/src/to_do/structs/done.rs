use super::base::*;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let super_struct = Base {
            title: input_title.to_string(),
            status: "done".to_string(),
        };
        Self { super_struct }
    }
}
