fn main() {
    /* let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number); */

    /* let n = 6;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    } */

    /* for i in 1..=5 {
        println!("{}", i);
    }

    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("第 {} 个元素是 {}", i + 1, v);
    }

    for _ in 0..10 {}

    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    } */

    /* let mut n = 0;

    while n <= 5 {
        println!("{}", n);
        n = n + 1;
    }
    println!("我出来了！"); */

    /* let mut n = 0;
    loop {
        if n > 5 {
            break;
        }
        println!("{}", n);
        n = n + 1;
    } */

    /* let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);
        index = index + 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    } */

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);
}
