#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}
enum IpAddr {
    Ipv4,
    Ipv6,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    /* let dire: Direction = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    } */

    /* let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str); */

    /*  let actions = [
        Action::Say("Hello rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!("Change color to RGB({}, {}, 0)", r, g);
            }
        }
    } */

    /* let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    } */

    /* let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    }; */

    /* let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(v) = v {
        println!("three");
    } */

    /* #[derive(Debug)]
    enum MyEnun {
        Foo,
        Bar,
    }

    let v = vec![MyEnun::Foo, MyEnun::Bar, MyEnun::Foo];
    v.iter().filter(|x| matches!(x, MyEnun::Foo)); */

    /* let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z')); */

    /* let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }
    println!("在匹配后，age是{:?}", age); */

    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    match age {
        Some(age) => println!("匹配出来的age是{}", age),
        _ => (),
    }
    println!("在匹配后，age是{:?}", age);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
