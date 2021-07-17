extern crate wasm_bindgen;
extern crate web_sys;
extern crate fib;

use fib::fib as fib_impl;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fib(n: i32) -> String  {
    String::from(format!("{}", fib_impl(n)))
}
