fn plus_five(x: i32) -> i32 {
    x + 5
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

fn main() {
    // another_function(5, 6.1);

    /* let x = plus_five(5);
    println!("The value of x is: {}", x); */

    let x = plus_or_minus(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

use std::fmt::Debug;

fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn dead_end() -> ! {
    panic!("This is the end of the road!");
}

fn forever() -> ! {
    loop {}
}
