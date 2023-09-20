struct MainEntity {
    rows: Vec<DataType>,
}

impl MainEntity {
    fn print_values(&self) {
        for row in &self.rows {
            match row  {
                DataType::U8(data) => data.duplicate_and_print(),
                DataType::F32(data) => data.duplicate_and_print(),
                DataType::Extra(data) => data.duplicate_and_print_for_strings(),
            }
        }
    }
    fn get_row_by_name(&self, name: &str) -> Option<DataType> {
        for row in self.rows.iter() {
            let row_clone = row.clone();
            match row {
                DataType::U8(row_data) => {
                    if row_data.name == name {
                        return Option::Some(row_clone);
                    }
                },
                DataType::F32(row_data) => {
                    if row_data.name == name {
                        return Option::Some(row_clone);
                    }
                },
                DataType::Extra(row_data) => {
                    if row_data.name == name {
                        return Option::Some(row_clone);
                    }
                },
            }
        }
        Option::None
    }
}

#[derive(Clone)]
enum DataType {
    U8(Data<u8>),
    F32(Data<f32>),
    Extra(ExtraStruct),
}

#[derive(Clone)]
struct Data<T> {
    name: String,
    values: Vec<T>,
}

#[derive(Clone)]
struct ExtraStruct {
    name: String,
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
    let row1 = DataType::U8(Data { name: "a".to_string(), values: vec![1_u8, 2_u8, 5_u8] });
    let row2 = DataType::F32(Data { name: "b".to_string(), values: vec![1.0_f32, 2.0_f32, 5.0_f32] });
    let row3 = ExtraStruct { name: "c".to_string(), values: vec!["alpha".to_string(), "beta".to_string() ]};
    let mainentity = MainEntity { rows: vec![row1, row2, DataType::Extra(row3)] };
    mainentity.print_values();
    let row1_clone = mainentity.get_row_by_name("a").unwrap();
}
