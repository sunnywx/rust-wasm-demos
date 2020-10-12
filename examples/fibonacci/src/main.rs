fn main() {
    println!("Fibnacci numbers:");

    // println!("fib(-1): {}", fib(-1)); // raise panic

    println!("fib(10): {}", fib(10));
    println!("fib(40): {}", fib(40));
}

fn fib(n: i32) -> u64 {
    if n < 0 {
        panic!("fib param should not be negative");
    }

    if n == 0 || n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}
