#[no_mangle]
/*
 fixme: no_mangle attr is required if you want exports rust fn to js
*/
pub fn add_one(x: u32) -> u32 {
    return x + 1;
}

#[no_mangle]
fn fib(n: i32) -> u64 {
    if n < 0 {
        panic!("fib param should not be negative");
    }

    // if n == 0 || n == 1 {
    //     return 1;
    // }
    //
    // return fib(n - 1) + fib(n - 2);

    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
