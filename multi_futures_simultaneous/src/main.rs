use futures::try_join;

async fn get_book() -> Result<Book, String> {
    /* ... */
    Ok(Book)
}
async fn get_music() -> Result<Music, String> {
    /* ... */
    Ok(Music)
}

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book();
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}

fn main() {
    println!("Hello, world!");
}
