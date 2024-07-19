/* #[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
} */

trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0);
    }
}

struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0);
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

fn main() {
    /* let v: Vec<i32> = Vec::new();

    let mut v = Vec::new();
    v.push(1);

    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(1);

    {
        let v = vec![1, 2, 3];

        // ...
    } // <- v超出作用域并在此处被删除

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("没有第三个元素"),
    } */

    /* let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    let first = &v[0];
    println!("The first element is: {first}"); */

    /* let v = vec![1, 2, 3];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10;
    } */

    /* let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in v {
        show_addr(ip);
    } */

    /* let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    } */

    /* let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);
    println!("Vector 长度是：{}，容量是：{}", v.len(), v.capacity());

    v.reserve(100);
    println!(
        "Vector（reserve）长度是：{}，容量是：{}",
        v.len(),
        v.capacity()
    );

    v.shrink_to_fit();
    println!(
        "Vector（shrink_to_fit）长度是：{}，容量是：{}",
        v.len(),
        v.capacity()
    ); */

    /* let mut v = vec![1, 2];
    assert!(!v.is_empty());

    v.insert(2, 3);
    assert_eq!(v.remove(1), 2);
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.pop(), Some(1));
    assert_eq!(v.pop(), None);
    v.clear();

    let mut v1 = [11, 22].to_vec();
    v.append(&mut v1);
    v.truncate(1);
    v.retain(|x| *x > 10);

    let mut v = vec![11, 22, 33, 44, 55];
    let mut m: Vec<_> = v.drain(1..=3).collect();

    let v2 = m.split_off(1);

    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]); */

    /* let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    // vec.sort_unstable();
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]); */

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
    ];

    people.sort_unstable();
    println!("{:?}", people);
}

/* fn show_addr(ip: IpAddr) {
    println!("{:?}", ip);
}
 */
