pub fn run_basic() {
    do_something_w_a_number(1_u8);
    do_something_w_a_number(1_u16);
    do_something_w_a_number(1_u32);
    do_something_w_a_number(1_f32);
    do_something_w_a_number(1_f64);
}


fn do_something_w_a_number<T>(_num: T) {
    println!("running basic.");
}
