use super::base::*;
use super::traits::{
    get::Get,
    edit::Edit,
    delete::Delete,
};

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let super_struct = Base::new(input_title, "done");
        Self { super_struct }
    }
}

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}
