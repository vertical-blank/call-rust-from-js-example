pub fn fib(n: i32) -> i128 {
    fn local(n: i32, a: i128, b: i128) -> i128 {
        if n == 0 {
            a
        } else {
            local(n - 1, b, a + b)
        }
    }

    local(n, 0, 1)
}
