extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello(count: i32) -> bool {
    if count < 0 {
        web_sys::console::log_1(&JsValue::from("A negative count is not supported."));
        return false;
    }

    for i in 0..count {
        web_sys::console::log_1(&JsValue::from(format!("Hello, World: {}", i)));
    }

    return true;
}
