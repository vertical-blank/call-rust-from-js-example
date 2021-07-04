#[no_mangle]
pub extern "C" fn hello(count: i32) -> bool {
    if count < 0 {
        println!("A negative count is not supported.");
        return false;
    }

    for i in 0..count {
        println!("Hello, World: {}", i);
    }

    return true;
}
