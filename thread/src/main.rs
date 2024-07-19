use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::sync::{Arc, Barrier, Condvar, Mutex, Once};
use std::thread::{self, LocalKey};
use std::time::Duration;
use thread_local::ThreadLocal;

static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn main() {
    /* let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    } */

    /* let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // println!("{:?}", v); */

    /* let new_thread = thread::spawn(move || {
        thread::spawn(move || loop {
            println!("I am a new thread.");
        })
    });

    new_thread.join().unwrap();
    println!("Child thread is finish!");

    thread::sleep(Duration::from_millis(100)); */

    /* let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    } */

    /* thread_local!(static FOO:RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        })
    });

    t.join().unwrap();

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    }) */

    /* Foo::FOO.with(|x| println!("{:?}", x)); */

    /* let tls = Arc::new(ThreadLocal::new());
    let mut v = vec![];
    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        });
        v.push(handle);
    }

    for handle in v {
        handle.join().unwrap();
    }

    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        println!("x: {}, y: {}", x, y.get());
        x + y.get()
    });

    assert_eq!(total, 5); */

    /* let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut statred = lock.lock().unwrap();
    while !*statred {
        statred = cvar.wait(statred).unwrap();
    }

    println!("statred changed"); */

    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}

/* struct Foo;

impl Foo {
    thread_local! {
        static FOO:RefCell<usize> = RefCell::new(0);
    }
}

thread_local! {
    static FOO:RefCell<usize> = RefCell::new(0);
}

struct Bar {
    foo: &'static LocalKey<RefCell<usize>>,
}

impl Bar {
    fn constructor() -> Self {
        Self { foo: &FOO }
    }
}
 */
