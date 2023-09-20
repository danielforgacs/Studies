mod basic;
mod multiple_types_in_vector;
mod generic_struct;
mod impl_vs_dyn;
mod generic_struct_two;
mod generic_struct_three;
mod generic_struct_four;
mod generic_nested_structs;
mod generic_nested_structs_two;

fn main() {
    basic::run_basic();
    multiple_types_in_vector::vec_multi_type();
    multiple_types_in_vector::vec_multi_type_w_trait();
    generic_struct::generic_struct();
    impl_vs_dyn::impl_vs_dyn();
    generic_struct_two::generic_struct_two();
    generic_struct_three::generic_struct_three();
    generic_struct_four::generic_struct_four();
    generic_nested_structs::generic_nested_structs();
    generic_nested_structs_two::run();
}
