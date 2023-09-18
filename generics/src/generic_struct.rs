struct AnyNum<T> {
    numbers: Vec<T>
}

pub fn generic_struct() {
    let stuff = AnyNum {
        numbers: vec![1, 2, 3],
    };
}
