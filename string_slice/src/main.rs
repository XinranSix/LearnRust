fn main() {
    /* let s = String::from("hello world");

    let hello = &[0..5];
    let world = &[6..11]; */

    /* let s = String::from("hello");
    let slice = &[0..2];
    let slice = &[..2]; */

    /* let s = String::from("hello");

    let len = s.len();

    let slice = &s[4..len];
    let slice = &s[4..]; */

    /* let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..]; */

    /* let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error
    println!("the first word is: {}", word); */

    /* let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]); */

    /* let s = "Hello, world!"; */

    /* let s1 = String::from("hello,world");
    let s2 = "hello,world".to_string(); */

    let s = String::from("hello, world");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn say_hello(s: &str) {
    println!("{}", s);
}
