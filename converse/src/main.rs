use std::sync::Arc;

fn main() {
    /* let a: i32 = 10;
    let b: u16 = 100;

    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    } */

    /* let a = i8::MAX;
    println!("{}", a); */

    /* let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("{}, {}, {}", a, b, c); */

    /* let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }

    assert_eq!(values[1], 3); */

    /* let a: u8 = 10;
    let b: u16 = 1500;

    let b_: u8 = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    } */

    let b: i16 = 1500;

    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
}

struct Foo {
    x: u32,
    y: u16,
}

struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}
fn do_stuff1<T: Clone>(value: &T) {
    let cloned = value.clone();
}

fn do_stuff2<T>(value: &T) {
    let cloned = value.clone();
}

#[derive(Clone)]
struct Container<T>(Arc<T>);

fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_cloned = foo.clone();
    let bar_cloned = bar.clone();
}
