struct Group {
    rows: Vec<ValueRow>,
}

struct ValueRow {
    values: Vec<Box<dyn std::fmt::Display>>,
}

pub fn generic_struct_three() {
    let row1 = ValueRow { values: vec![Box::new(1_u8), Box::new(2_u8), Box::new(5_u8)] };
    let row2 = ValueRow { values: vec![Box::new(1.0_f32), Box::new(2.0_f32), Box::new(5.0_f32)] };
    let group = Group { rows: vec![row1, row2] };
    for row in group.rows {
        for value in row.values {
            println!("value: {}", value);
        }
    }
}
