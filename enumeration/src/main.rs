/* #[derive(Debug)]
enum PokerSuit {
    Clubes,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8,
}
 */

/* enum PokerCard {
    Clubes(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}
 */

use std::net::TcpStream;

enum PokerCard {
    Clubes(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    /* let heart = PokerSuit::Hearts;
    let dimond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(dimond); */

    /* let c1 = PokerCard {
        suit: PokerSuit::Clubes,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    }; */

    /* let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds(13); */

    /* let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds('A'); */

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m3 = Message::ChangeColor(255, 255, 0);

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

/* fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}
 */
