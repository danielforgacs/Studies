struct MainEntity {
    rows: Vec<DataType>,
}

enum DataType {
    U8(Data<u8>),
    F32(Data<f32>),
    Extra(ExtraStruct),
}

struct Data<T> {
    values: Vec<T>,
}

struct ExtraStruct {
    values: Vec<String>,
}

impl<T> Data<T>
where
    T: std::fmt::Display + std::fmt::Debug
{
    fn duplicate_and_print(&self) {
        for value in &self.values {
            print!("{} <{:?}>, ", value, value);
        }
        println!();
    }
}

impl ExtraStruct {
    fn duplicate_and_print_for_strings(&self) {
        for value in &self.values {
            println!("extrastuct value: {}", value);
        }
    }
}

pub fn generic_nested_structs() {
    println!("generic_nested_structs");
    let row1 = DataType::U8(Data { values: vec![1_u8, 2_u8, 5_u8] });
    let row2 = DataType::F32(Data { values: vec![1.0_f32, 2.0_f32, 5.0_f32] });
    let row3 = ExtraStruct { values: vec!["alpha".to_string(), "beta".to_string() ]};
    let group = MainEntity { rows: vec![row1, row2, DataType::Extra(row3)] };
    for row_values in group.rows {
        match row_values {
            DataType::U8(values) => values.duplicate_and_print(),
            DataType::F32(values) => values.duplicate_and_print(),
            DataType::Extra(values) => values.duplicate_and_print_for_strings(),
        }
    }
}
