fn main() {
    /* let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); */

    /* let s1 = String::from("hello");
    let len = calulate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); */

    /* let mut s = String::from("hello");
    change(&mut s); */

    /* let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2); */

    /* let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s; */

    /* let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题

    println!("{}, {}, and {}", r1, r2, r3); */

    /* let reference_to_nothing = dangle(); */
}

fn calulate_length(s: &String) -> usize {
    s.len()
}

fn change(some_thing: &mut String) {
    some_thing.push_str(", world");
}

/* fn dangle() -> &String {
    let s = String::from("hello");

    &s
} */
