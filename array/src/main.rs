use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    /* let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    let a = [9, 8, 7, 6, 5];
    let first = a[0];
    let second = a[1]; */

    /* let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index emtered was not a number");

    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    ); */

    /* // let array = [String::from("rust is good"); 8]; // Error
    let array = [
        String::from("rust is good!"),
        String::from("rust is good!"),
        String::from("rust is good!"),
    ];
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:#?}", array); */

    /* let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]); */

    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    for a in &arrays {
        for n in a.iter() {
            print!("\t{} = 10 + {}", n, n + 10);
        }
        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
