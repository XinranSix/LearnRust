fn main() {
    /* let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); */

    /* let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len); */

    /* let mut s = String::from("hello");
    change(&mut s); */

    /* let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2); */

    /* let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s; */

    /* let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{}, {}, and {}", r1, r2, r3); */

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/* fn dangle() -> &String {
    let s = String::from("hello");
    &s
} */

fn dangle() -> String {
    let s = String::from("hello");
    s
}
