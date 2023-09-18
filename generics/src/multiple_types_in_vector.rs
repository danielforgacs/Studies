use core::fmt::Debug;

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
