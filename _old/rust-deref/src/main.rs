fn main() {
    let a = 2;
    let mut b = 3;
    just_some_func(&a, &mut b);
    println!("a: {}", a);
    println!("b: {}", b);
    // just_some_func_no_deref(&a, &b);
}

fn just_some_func(input: &u32, sum: &mut u32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

/*
THIS ONE CRASHES FOR LIFETIME REASONS

fn just_some_func_no_deref(input: &u32, mut sum: &u32) {
    let t = input + input;
    let t_ref = &t.clone();
    sum = &t_ref.clone();
}
*/
