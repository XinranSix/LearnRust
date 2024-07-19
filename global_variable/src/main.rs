use lazy_static::lazy_static;
use std::{
    char::MAX,
    collections::HashMap,
    panic,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};
use std::{sync::OnceLock, thread};

// const MAX_ID: usize = usize::MAX / 2;

// static mut REQUEST_RECV: usize = 0;
// static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);

struct Factory {
    factory_id: usize,
}

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

fn generate_id() -> usize {
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory ids overflowed");
    }
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overflowed");
    }
    next_id
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }
}

lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
}

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    Some(Box::leak(c))
}

fn main() {
    // println!("用户 ID 允许的最大值是 {}", MAX_ID);

    /* unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
    } */

    /* for _ in 0..100 {
        REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    }

    println!("当前用户请求数 {:?}", REQUEST_RECV); */

    /* let mut v = NAMES.lock().unwrap();
    v.push_str(", Myth");
    println!("{}", v); */

    /*  // 首次访问`HASHMAP`的同时对其进行初始化
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    // 后续的访问仅仅获取值，再不会进行任何初始化操作
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap()); */

    /* let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        CONFIG = Some(Box::leak(c));
        println!("{:?}", CONFIG);
    } */

    /* unsafe {
        CONFIG = init();

        println!("{:?}", CONFIG)
    } */

    let handle = thread::spawn(|| {
        let logger = Logger::global();
        logger.log("thread message".to_string());
    });

    let logger = Logger::global();
    logger.log("some message".to_string());

    let logger2 = Logger::global();
    logger2.log("other message".to_string());

    handle.join().unwrap();
}

#[derive(Debug)]
struct Logger;

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    fn global() -> &'static Logger {
        LOGGER.get_or_init(|| {
            println!("Logger is being created...");
            Logger
        })
    }

    fn log(&self, message: String) {
        println!("{}", message);
    }
}
