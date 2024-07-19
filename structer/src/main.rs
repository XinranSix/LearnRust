#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    /* let user1 = User {
           email: String::from("someone@example.com"),
           username: String::from("someusername123"),
           active: true,
           sign_in_count: 1,
       };
    */
    /* let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

     let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    }; */

    /* let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user1.active);
    // 下面这行会报错
    println!("{:?}", user1.username); */

    /* let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length); */

    /* struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); */

    /* #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); */

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}

fn build_user(email: String, username: String) -> User {
    /* User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    } */

    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
