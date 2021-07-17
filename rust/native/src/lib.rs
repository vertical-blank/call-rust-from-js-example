extern crate fib;
extern crate libc;

use libc::c_char;
use std::ffi::CString;
use fib::fib as fib_impl;

#[no_mangle]
pub extern "C" fn fib(n: i32) -> *mut c_char {
    CString::new(String::from(format!("{}", fib_impl(n)))).unwrap().into_raw()
}
