mod basic;
mod multiple_types_in_vector;
mod generic_struct;
mod impl_vs_dyn;

fn main() {
    basic::run_basic();
    multiple_types_in_vector::vec_multi_type();
    multiple_types_in_vector::vec_multi_type_w_trait();
    generic_struct::generic_struct();
    impl_vs_dyn::impl_vs_dyn()
}
