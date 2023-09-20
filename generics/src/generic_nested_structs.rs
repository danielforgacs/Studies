struct MainEntity {
    rows: Vec<DataType>,
}

enum DataType {
    U8(Data<u8>),
    F32(Data<f32>),
}

struct Data<T> {
    values: Vec<T>,
}

impl<T: std::fmt::Display + std::fmt::Debug> Data<T> {
    fn duplicate_and_print(&self) {
        for value in &self.values {
            print!("{} <{:?}>, ", value, value);
        }
    }
}

pub fn generic_nested_structs() {
    println!("generic_nested_structs");
    let row1 = DataType::U8(Data { values: vec![1_u8, 2_u8, 5_u8] });
    let row2 = DataType::F32(Data { values: vec![1.0_f32, 2.0_f32, 5.0_f32] });
    let group = MainEntity { rows: vec![row1, row2] };
    for row_values in group.rows {
        match row_values {
            DataType::U8(values) => values.duplicate_and_print(),
            DataType::F32(values) => values.duplicate_and_print(),
        }
    }
}
