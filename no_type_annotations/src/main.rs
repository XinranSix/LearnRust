fn main() {
    let guess = "42".parse().expect("Not a number!");
    let guess = "42".parse::<i32>().expect("Not a number!");
    let guess: i32 = "42".parse().expect("Not a number!");
}
