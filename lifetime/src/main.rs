#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    /* {
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!(":r {}", r);
    }

    {
        let x = 5;

        let r = &x;

        println!("r: {}", r);
    } */

    /* let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1.as_str(), string2);
    println!("The longest string is {}", result); */

    /* let string1 = String::from("long string is long");

       {
           let string2 = String::from("xyz");
           let result = longest(string1.as_str(), string2.as_str());
           println!("The longest string is {}", result);
       }
    */

    /* let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); */

    /* let novel = String::from("Call m Ishmael. Some years age...");
    let first_sentence = novel.split('.').next().expect("Counld not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    }; */

    /* let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{:?}", i); */

    // let s: &'static str = "I have a static lifetime.";
}

/* fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn uneless<'a>(first: &'a u32, second: &'a i32) {}
 */

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
