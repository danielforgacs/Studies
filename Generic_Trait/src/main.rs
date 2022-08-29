#[derive(Debug)]
struct A;

impl A {
    fn print_self(&self) {
        println!("impl print a: {:?}", self);
    }
}

trait GenericPrint {
    fn genp(&self)
    where
        Self: std::fmt::Debug
    {
        println!("generic print: {:?}, cloned: {:?}", self, self.clone());
    }
}

impl GenericPrint for A {}

fn generic_func<T>(item: &T) -> [&T; 2] {
    [&item, &item]
}

fn main() {
    let a = A {};
    a.print_self();
    a.genp();
    println!("items: {:?}", generic_func(&a));
    println!("items: {:?}", generic_func::<A>(&a));
}
