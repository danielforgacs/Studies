use rand::prelude::*;

fn generate_float(generator: &mut ThreadRng) -> f64 {
    let placeholder: f64 = generator.gen();
    return placeholder * 10.0;
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = generate_float(&mut rng);
    println!("generated random num: {}", random_number);
}
