fn main() {
    /* let my_name = "Pascal";
    greet(my_name); */

    /* let s = String::from("hello world");
    let hello = &[0..5];
    let world = &[6..11]; */

    /* let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2]; */

    /* let s = String::from("hello");
    let len = s.len();
    let slice = &s[4..len];
    let slice = &s[4..]; */

    /* let s = "中国人";
    let a = &s[0..2]; // panic
    println!("{}", a); */

    /* let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear();
    println!("the first word is: {}", word); */

    /* let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]); */

    /* let s = "Hello, world!";
    let s: &str = "Hello, world!"; */

    /* let s = String::from("hello, world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str()); */

    /* let s1 = String::from("hello");
    let h = s1[0]; */

    /* let hello = "中国人";
    let s = &hello[0..2]; */

    /* let mut s = String::from("hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);
    s.push('!');
    println!("追加字符 push() -> {}", s);

    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replace = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replace);

    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    println!("连接字符串 + -> {}", result);

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello, world!");
    // println!("{}", s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s); */

    // 字符串转义
    /* let bytes_escape = "I'm writing \x52\x75\x73\x74";
    println!("what are you doing\x3F (\\x3F means ?) {}", bytes_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) us called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                    can span multiple lines.
                    The linebreak and indentation here ->\
                    <- will become part of the string.";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter); */

    /* for c in "中国人".chars() {
        println!("{}", c);
    }

    for b in "中国人".bytes() {
        println!("{}", b);
    } */

    use utf8_slice;

    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    println!("{}", rocket)
}

#[allow(dead_code)]
fn greet(name: String) {
    println!("Hello,  {}!", name);
}

#[allow(dead_code)]
fn first_word(s: &String) -> &str {
    &s[..1]
}

#[allow(dead_code)]
fn say_hello(s: &str) {
    println!("{}", s);
}
