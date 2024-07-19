use std::ops::Deref;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn display(&mut self, age: u8) {
        let Person { name, age } = &self;
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    /* let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); */

    /* let x = Box::new(1);
    let sum = *x + 1; */

    /* let y = MyBox::new(5);
    assert_eq!(5, *y); */

    let s = String::from("hello world");
    display(&s);

    let s = MyBox::new(String::from("hello world"));
    display(&s);

    let s = MyBox::new(String::from("hello, world"));
    let s1: &str = &s;
    let s2: String = s.to_string();
}

fn display(s: &str) {
    println!("{}", s);
}
