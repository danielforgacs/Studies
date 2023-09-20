trait Rnd {}

/// This syntax is coming from the compiler error
trait AutoTraitForDyn: Rnd + std::fmt::Display {}

impl Rnd for u8 {}
impl Rnd for f32 {}

impl AutoTraitForDyn for u8 {}
impl AutoTraitForDyn for f32 {}

pub fn impl_vs_dyn() {
    trait_as_impl_arg(1_u8);
    trait_as_impl_arg(1.0_f32);
    trait_as_dyn_arg(&1_u8);
    trait_as_dyn_arg(&1.0_f32);
}

fn trait_as_impl_arg(k: impl Rnd + std::fmt::Display) {
    println!("trait_as_impl_arg: {}", k);
}

/// "dyn" alwalys has to be & because we don't know
/// the trait size at compile time
fn trait_as_dyn_arg(k: &dyn AutoTraitForDyn) {
    println!("trait_as_dyn_arg: {}", k);
}
