mod basic;
mod multiple_types_in_vector;
mod generic_struct;

fn main() {
    basic::run_basic();
    multiple_types_in_vector::vec_multi_type();
    multiple_types_in_vector::vec_multi_type_w_trait();
    generic_struct::generic_struct();
}
