use std::fmt::Debug;

fn main() {
    /* anther_function(5, 6.1); */

    /* let x = plus_five(5);
    println!("The value of x is: {}", x); */

    let x = plus_or_minus(5);
    println!("The value of x is: {}", x);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn anther_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}

fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

/* fn add(x:u32,y:u32) -> u32 {
    x + y;
}
 */

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

fn forever() -> ! {
    loop {
        //...
    }
}
