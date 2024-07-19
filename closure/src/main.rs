use std::thread;
use std::time::Duration;

fn workout(intensity: u32, random_number: u32) {
    let action = || {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("今天活力满满，先做 {} 个俯卧撑!", action());
        println!("旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!", action());
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!("昨天练过度了，今天干干有氧，跑步 {} 分钟!", action());
    }
}

struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Self { query, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    /* // 强度
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout(intensity, random_number); */

    // let sum = |x: i32, y: i32| -> i32 { x + y };
    /* let sum = |x, y| x + y;
    let v = sum(1, 2); */

    /* fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1; */

    /* let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y)); */

    /* let x = vec![1, 2, 3];
    fn_once(|z| z == x.len()); */

    /* use std::thread;
    let v = vec![1, 2, 3];
    let headle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    headle.join().unwrap(); */

    /* let mut s = String::new();

    let mut update_string = |str| s.push_str(str);
    update_string("hello");

    println!("{:?}", s); */

    /* let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s); */

    /* let s = String::new();
    let update_string = || println!("{}", s);

    let mut s = String::new();
    let mut update_string = || s.push_str("hello"); */

    /* let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s); */

    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;

        if x > 1 {
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x - num)
        }
    }
}

/* fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
} */

fn exec<'a, F: Fn(&'a str)>(mut f: F) {
    f("hello")
}
