#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

impl A {
    fn print_self(&self) {
        println!("impl print a: {:?}", self);
    }
}

impl B {
    fn print_self(&self) {
        println!("impl print B: {:?}", self);
    }
}

trait GenericPrint {
    fn genp(&self)
    where
        Self: std::fmt::Debug
    {
        println!("generic print: {:?}", self);
    }
}

fn main() {
    let a = A {};
    let b = B {};

    a.print_self();
    b.print_self();
}
