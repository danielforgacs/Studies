use super::base::*;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let super_struct = Base::new(input_title, "pending");
        Self { super_struct }
    }
}
