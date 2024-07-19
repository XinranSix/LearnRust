use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    /* let s = String::from("hello, world");
    let a = Box::new(s);
    // let b = Box::new(s); */

    /* let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b)); */

    /* let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); */

    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            println!("{}", s);
        });
    }
}
