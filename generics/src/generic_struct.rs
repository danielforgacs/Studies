struct AnyNum<T> {
    numbers: Vec<T>
}

pub fn generic_struct() {
    print!("Generic struct");
    let stuff_0 = AnyNum {
        numbers: vec![1_u8, 2_u8, 3_u8],
    };
    let stuff_1 = AnyNum {
        numbers: vec![1_f32, 2_f32, 3_f32],
    };
}
