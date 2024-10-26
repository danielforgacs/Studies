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

pub fn generic_struct_four() {
    let row1 = DataType::U8(Data { values: vec![1_u8, 2_u8, 5_u8] });
    let row2 = DataType::F32(Data { values: vec![1.0_f32, 2.0_f32, 5.0_f32] });
    let group = MainEntity { rows: vec![row1, row2] };
    for row_values in group.rows {
        match row_values {
            DataType::U8(values) => do_stuff_with_values::<u8>(values),
            DataType::F32(values) => do_stuff_with_values::<f32>(values),
        }
    }
}

fn do_stuff_with_values<T>(values: Data<T>)
where
    T: std::fmt::Debug
{
    for k in values.values {
        println!("{:?}", k);
    }
}
