struct MainEntity {
    rows: Vec<Data>,
}

struct Data {
    name: String,
    values: Vec<u8>,
}

pub fn run() {
    println!("generic_nested_structs two");
    let mainentity = MainEntity { rows: vec![
        Data { name: "a".to_string(), values: vec![1_u8, 2_u8, 5_u8] },
        // Data { name: "b".to_string(), values: vec![1.0_f32, 2.0_f32, 5.0_f32] },
    ]};
}
