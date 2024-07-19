use futures::executor::block_on;

/* async fn do_something() {
    println!("go go go !");
}

async fn hello_world() {
    hello_cat().await;
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, kitty!");
} */

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} - {}",
        song.author, song.name, "你存在我深深的脑海里"
    );
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

fn main() {
    /* let future = hello_world();
    block_on(future); */

    /* let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance()); */

    block_on(async_main());
}
