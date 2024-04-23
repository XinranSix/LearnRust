fn main() {
    /* {
        // s 在这里无效，它尚未声明
        let s = "hello"; // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，s不再有效

    let mut s = String::from("hello");
    s.push_str(" world!");
    println!("{}", s); */

    /* let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world", s1); */

    /* let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y); */

    /* let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); */

    /* let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); */

    /*  let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x

    // println!("在move进函数后继续使用s: {}", s); */

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {
    let some_thing = String::from("hello");
    some_thing
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
