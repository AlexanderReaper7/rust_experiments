use std::time::Duration;

use scrap::*;

pub fn run_speed_test(iterations: u32) {
    let mut capturer = Capturer::new(Display::primary().unwrap()).unwrap();
    let mut image;
    println!("width: {:4}, height: {:4}", capturer.width(), capturer.height());
    while capturer.frame().is_err() {std::thread::sleep(std::time::Duration::from_millis(1));}
    println!("got first frame");
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    let mut iteration = 0;
    let MIN_FRAME_TIME = std::time::Duration::from_secs_f32(1.0 / 20.0);
    while iteration < iterations {
        let start = std::time::Instant::now();
        match capturer.frame() {
            Ok(frame) => {
                image = frame;
            },
            Err(_) => {
                continue;
            }
        }
        iteration += 1;
        if start.elapsed() < MIN_FRAME_TIME {
            std::thread::sleep(MIN_FRAME_TIME - start.elapsed());
        }
        let time = 1.0 / start.elapsed().as_secs_f32();
        min = min.min(time);
        max = max.max(time);
        println!("iteration {}, {:.0} hz", iteration, time);
    }
    println!("min fps: {}", min);
    println!("max fps: {}", max);
}