fn main() {
    for i in 0..10 {
        println!("{}", fib(i));
    }
}

fn fib(n: i32) -> i32 {
    fn local(n: i32, a: i32, b: i32) -> i32 {
        if n == 0 {
            a
        } else {
            local(n - 1, b, a + b)
        }
    }

    local(n, 0, 1)
}

