use super::base::*;
use super::traits::{
    create::Create,
    get::Get,
    edit::Edit,
    delete::Delete,
};

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let super_struct = Base::new(input_title, "pending");
        Self { super_struct }
    }
}

impl Create for Pending {}
impl Get for Pending {}
impl Edit for Pending {}
impl Delete for Pending {}
