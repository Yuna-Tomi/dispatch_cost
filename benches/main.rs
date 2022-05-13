#![feature(test)]
extern crate test;
use test::Bencher;

use dispatch_cost::{Greet, Greeter1, Greeter2};
use paste;

fn static_greet1(g: &Greeter1, n: isize) {
    g.hello(n);
    if n > 0 {
        static_greet1(g, n-1);
    } else {
        g.bye();
    }
}

fn static_greet2(g: &Greeter2, n: isize) {
    g.hello(n);
    if n > 0 {
        static_greet2(g, n-1);
    } else {
        g.bye();
    }
}

fn dyn_greet(g: &dyn Greet, n: isize) {
    g.hello(n);
    if n > 0 {
        dyn_greet(g, n-1);
    } else {
        g.bye();
    }
}

const N_RECUR: isize = 10000;

macro_rules! bench {
    ($($n:literal),* $(,)?) => {
        $(
            paste::paste!{
                #[bench]
                fn [<static_dispatch1 _ $n>](b: &mut Bencher) {
                    let greeter = &Greeter1;
                    b.iter(|| static_greet1(greeter, N_RECUR));
                }
                
                #[bench]
                fn [<static_dispatch2 _ $n>](b: &mut Bencher) {
                    let greeter = &Greeter2;
                    b.iter(|| static_greet2(greeter, N_RECUR));
                }
                
                #[bench]
                fn [<dyn_dispatch1 _ $n>](b: &mut Bencher) {
                    let greeter = &Greeter1 as &dyn Greet;
                    b.iter(|| dyn_greet(greeter, N_RECUR));
                }
                
                #[bench]
                fn [<dyn_dispatch2 _ $n>](b: &mut Bencher) {
                    let greeter = &Greeter2 as &dyn Greet;
                    b.iter(|| dyn_greet(greeter, N_RECUR));
                }
            }
        )*
    };
}


bench!(100, 10000, 1000000);