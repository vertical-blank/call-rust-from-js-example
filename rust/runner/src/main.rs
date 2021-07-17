extern crate fib;

use std::env;
use fib::fib as fib_impl;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n = &args[1].parse::<i32>().unwrap();
    let fib = fib_impl(*n);
    println!("{}", fib);
}
