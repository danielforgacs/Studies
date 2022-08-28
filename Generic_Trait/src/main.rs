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
        println!("generic print: {:?}, cloned: {:?}", self, self.clone());
        self.call_print_self();
    }

    fn call_print_self(&self);
    fn get_instance(self) -> Self;
}

impl GenericPrint for A {
    fn call_print_self(&self) {
        self.print_self();
    }

    fn get_instance(self) -> Self {
        self
    }
}

impl GenericPrint for B {
    fn call_print_self(&self) {
        self.print_self();
    }

    fn get_instance(self) -> Self {
        self
    }
}

fn generic_func<T>(item: &T) -> [&T; 2] {
    [&item, &item]
}

fn main() {
    let a = A {};
    let b = B {};

    a.print_self();
    b.print_self();

    a.genp();
    b.genp();

    println!("items: {:?}", generic_func(&a));
    println!("items: {:?}", generic_func(&b));

    println!("items: {:?}", generic_func::<A>(&a));
    println!("items: {:?}", generic_func::<B>(&b));
}
