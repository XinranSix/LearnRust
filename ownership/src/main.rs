fn main() {
    /* let s = "hello";

    {
        let s = "hello";
    } */

    /* let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s); */

    /* let x = 5;
    println!("{}", x);
    let y = x; // no move but copy
    println!("{}", x);

    let s1 = String::from("hello");
    println!("{}", s1);
    let s2 = s1; // move
                 // let s2 = s1.clone(); // copy
                 // println!("{}", s1); */

    /* let x: &str = "hello, world";
    let y = x;
    println!("{}, {}", x, y); */

    /* let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2); */

    /* let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); */

    /* let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x); */

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
