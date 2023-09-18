use core::fmt::Debug;
use std::fmt::Formatter;

trait GroupThem {}

pub fn vec_multi_type() {
    let mut stuff: Vec<Box<dyn Debug>> = Vec::new();
    stuff.push(Box::new(1_u8));
    stuff.push(Box::new(2_u16));
    stuff.push(Box::new(3.0_f32));
    stuff.push(Box::new(4.0_f64));
    stuff.push(Box::new("str"));
    stuff.push(Box::new("String".to_string()));
    dbg!(&stuff);
}

pub fn vec_multi_type_w_trait() {
    let mut stuff: Vec<Box<dyn GroupThem>> = Vec::new();
    stuff.push(Box::new(1_u8));
    stuff.push(Box::new(2_u16));
    stuff.push(Box::new(3.0_f32));
    stuff.push(Box::new(4.0_f64));
    stuff.push(Box::new("str"));
    stuff.push(Box::new("String".to_string()));
    dbg!(&stuff);
}

impl GroupThem for u8 {}
impl GroupThem for u16 {}
impl GroupThem for f32 {}
impl GroupThem for f64 {}
impl GroupThem for &str {}
impl GroupThem for String {}

impl Debug for dyn GroupThem {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
