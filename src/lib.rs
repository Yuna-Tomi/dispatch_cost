pub trait Greet {
    fn hello(&self, n: isize) {
        eprintln!("Hello {}!", n);
    }

    fn bye(&self) {
        eprintln!("Bye!");
    }
}

pub struct Greeter1;
impl Greet for Greeter1 {}

pub struct Greeter2;
impl Greet for Greeter2 {}
