fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y //表达式
}

fn main() {
    // 语句
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);
    // let c = (let d = 8); // error

    /* let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y); */

    assert_eq!(ret_unit_type(), ())
}

fn ret_unit_type() {
    let x = 1;

    let y = if x % 2 == 1 { "odd" } else { "even" };

    let z = if x % 2 == 1 { "odd" } else { "even" };
}
