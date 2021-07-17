extern crate fib;

use fib::fib as fib_impl;

pub fn fibs(count: i32) -> bool {
    if count < 0 {
        println!("A negative count is not supported.");
        return false;
    }

    for i in 0..count {
        println!("{}:{}", i, fib_impl(i));
    }

    return true;
}

fn main() {
    fibs(10);
    let fib = fib_impl(100);
    println!("{}", fib);
}
