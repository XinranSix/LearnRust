use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;

fn main() {
    /* let mut my_gems = HashMap::new();

    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18); */

    /* let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    /* let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    } */

    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();

    println!("{:?}", teams_map); */

    /* let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);

    // std::mem::drop(name);
    println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    println!("还有，他的真实年龄远远不止{}岁", age); */

    /* let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);
    let score: i32 = scores.get(&team_name).copied().unwrap_or(0); */

    /* let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    } */

    /* let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5);

    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); */

    /* let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); */

    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
}
