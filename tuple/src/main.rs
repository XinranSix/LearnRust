fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_bundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let s1 = String::from("hello");
    let (s2, len) = calulate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calulate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
