extern crate wasm_bindgen;
extern crate web_sys;
extern crate fib;

use fib::fib as fib_impl;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibs(count: i32) -> bool {
    if count < 0 {
        web_sys::console::log_1(&JsValue::from("A negative count is not supported."));
        return false;
    }

    for i in 0..count {
        println!("{}:{}", i, fib_impl(i));
        web_sys::console::log_1(&JsValue::from(format!("{}:{}", i, fib_impl(i))));
    }

    return true;
}

#[wasm_bindgen]
pub fn fib(n: i32) -> String  {
    String::from(format!("{}", fib_impl(n)))
}
