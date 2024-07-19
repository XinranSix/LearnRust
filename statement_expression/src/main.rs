fn main() {
    /* let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false); */

    // let b = (let a = 8); // Error

    /* let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y); */
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; //语句
    let y = y + 1; // 语句
    x + y //表达式
}

fn ret_unit_type() {
    let x = 1;
    let y = if x % 2 == 1 { "odd" } else { "even" };
}
