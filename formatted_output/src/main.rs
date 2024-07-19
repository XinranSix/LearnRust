#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    /* println!("Hello");
    println!("Hello, {}!", "world");
    println!("The number is {}", 1);
    println!("{:?}", (3, 4));
    println!("{value}", value = 4);
    println!("{} {}", 1, 2);
    println!("{:04}", 42); */

    /* let s = "hello";
    println!("{}, world!", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!"); */

    // eprintln!("Error: Could not complete task");

    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };

    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);

    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
                              // => Alice, this is Bob. Bob, this is Alice
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{1}{}{0}{}", 1, 2); // => 2112

    println!("{argument}", argument = "test"); // => "test"
    println!("{name} {}", 1, name = 2); // => "2 1"
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"

    let v = 3.1415926;
    // Display => 3.14
    println!("{:.2}", v);
    // Debug => 3.14
    println!("{:.2?}", v);

    //-----------------------------------
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------

    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);
}
