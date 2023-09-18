struct AnyNum<T> {
    numbers: Vec<T>
}

impl<T> AnyNum<T> {
    fn new(nums: Vec<T>) -> Self {
        Self {
            numbers: nums,
        }
    }
}

pub fn generic_struct() {
    print!("Generic struct");
    let stuff_0 = AnyNum::new(vec![1_u8, 2_u8, 3_u8]);
    dbg!(stuff_0.numbers);
    let stuff_1 = AnyNum::new(vec![1_f32, 2_f32, 3_f32]);
    dbg!(stuff_1.numbers);
}
