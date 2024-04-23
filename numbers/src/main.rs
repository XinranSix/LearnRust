fn main() {
    /* assert_eq!(100u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX); */

    /* let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b); // 19 */

    /* let x = 2.0; // f64
    let y: f32 = 3.0; // f32 */

    // 断言0.1 + 0.2与0.3相等
    // assert!(0.1 + 0.2 == 0.3);
    // assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);

    /* let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("        0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2); */

    /* let x = (-42.0_f32).sqrt();
    assert_eq!(x, x); */

    /* let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    } */

    /* let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5; */

    /* let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];
    println!("{:.2}", forty_twos[0]); */

    // 位运算
    /* let a: i32 = 2;
    let b: i32 = 3;

    println!("(a & b) value is {}", a & b);
    println!("(a | v) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {}", !b);
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    a <<= b;
    println!("(a << b) value is {}", a); */

    /* for i in 1..5 {
        println!("{}", i);
    }
    println!();
    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    } */
}
