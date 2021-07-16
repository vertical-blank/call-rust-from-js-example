extern crate fib;
extern crate libc;

use libc::c_char;
use std::ffi::CString;
use fib::fib as fib_impl;

#[no_mangle]
pub extern "C" fn fibs(count: i32) -> bool {
    if count < 0 {
        println!("A negative count is not supported.");
        return false;
    }

    for i in 0..count {
        println!("{}:{}", i, fib_impl(i));
    }

    return true;
}

#[no_mangle]
pub extern "C" fn fib(n: i32) -> *mut c_char {
    // fib_impl(n)
    CString::new(String::from(format!("{}", fib_impl(n)))).unwrap().into_raw()
}
