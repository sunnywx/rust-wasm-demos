#[allow(unused_variables)]
fn main() {
    // println! is a macro, not function
    println!("hello world");

    let x = 1;

    // shadow variable
    let x = x + 2;

    let y = 7;

    // print formatted string
    println!("x is {}", x);

    println!("x is {0}, y is {1}", x, y);

    println!("x is {1}, y is {0}", y, x);

    // debug val with orig format, like tuple, array
    println!("debug {:?}, {:?}", x, (10, "sunny"));

    // pretty print
    println!("pretty print val: {:#?}", [1, 2, 3]);
}
