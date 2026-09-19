use rust_experiments_lib::*;
fn main() {
    // println!("starting asset embeding");
    // let text = String::from_utf8(assets::decompress(assets::EMBEDED_FILE)).unwrap();
    // println!("\n{}", text);

    // println!("starting channel_concurrency");
    // channel_concurrency::run2();

    // println!("starting screen capture test");
    // screen_capture::run_speed_test(100);

    // println!("starting hashing test");
    // hashing::run();

    // println!("starting tui test");
    // terminal_ui::main();

    // println!("starting archon shard probability test");
    // archon_shard_probability::main();

    println!("starting beggar my neighbor test");
    beggar_my_neighbor::main();

    pause_at_end()
}

fn pause_at_end() {
    println!("press enter to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}