use lazy_static::lazy_static;
use std::rc::Rc;
use std::time::Duration;
use std::{
    sync::{Arc, Condvar, Mutex, MutexGuard, RwLock},
    thread::{self, sleep, spawn},
};

use tokio::sync::Semaphore;

lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

#[tokio::main]
async fn main() {
    /* let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m); */

    /* let m = Mutex::new(5);
    let mut num = m.lock().unwrap();
    *num = 6;

    let mut num1 = m.lock().unwrap();
    *num1 = 7;

    println!("m = {:?}", m); */

    /* let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", *counter.lock().unwrap()); */

    /* let data = Mutex::new(0);
    let d1 = data.lock();
    let d2 = data.lock(); */

    /* let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            if i_thread % 2 == 0 {
                let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();
                println!("线程 {} 锁住了 MUTEX1，接着准备去锁 MUTEX2！", i_thread);
                sleep(Duration::from_millis(10));
                let guard = MUTEX2.lock().unwrap();
            } else {
                let _guard = MUTEX2.lock().unwrap();
                println!("线程 {} 锁住了 MUTEX2，接着准备去锁 MUTEX1！", i_thread);
                // let _guard = MUTEX1.lock().unwrap();
                let _guard = MUTEX1.try_lock();
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生"); */

    /* let lock = RwLock::new(5);

    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    } */

    /* let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            while !*lock {
                lock = ccond.wait(lock).unwrap();
            }

            *lock = false;

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;

    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag); */

    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
