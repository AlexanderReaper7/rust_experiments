use rust_experiments_lib::*;
fn main() {
    println!("starting asset embeding");
    let text = String::from_utf8(assets::decompress(assets::EMBEDED_FILE)).unwrap();
    println!("\n{}", text);

    println!("starting channel_concurrency");
    // let mut app = channel_concurrency::App::new();
    // app.run();
    channel_concurrency::run2();
    println!("press any key to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}