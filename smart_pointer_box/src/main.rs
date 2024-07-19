enum List {
    Cons(i32, Box<List>),
    Nil,
}

trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第 {} 号按钮", self.id);
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用 {}", self.id)
    }
}

fn main() {
    /* let b = foo("world");
    println!("{}", b); */

    /* let a = Box::new(3);
    println!("a = {}", a);

    let b = *a + 1; */

    /* let arr = [0; 1000];
    let arr1 = arr;

    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let arr = Box::new([0; 1000]);
    let arr1 = arr;

    // println!("{:?}", arr.len());
    println!("{:?}", arr1.len()); */

    /* let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    } */

    // let arr = vec![Box::new(1), Box::new(2)];
    // let (first, second) = (&arr[0], &arr[1]);
    // let sum = **first + **second;

    let s = gen_static_str();
    println!("{}", s);
}

fn foo(x: &str) -> String {
    let a = "Hello, ".to_string() + x;
    a
}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}
