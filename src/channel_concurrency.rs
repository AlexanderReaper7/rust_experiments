//! A test for multi threading with one consumer and one producer.
//! The producer will produce some item and will send it to the consumer at a diffrent thread.

use std::{sync::mpsc::{channel, Receiver}, thread};

pub struct App {
    receiver: Receiver<i32>,
}
impl App {
    pub fn new() -> Self {
        App {
            receiver: App::start_producer(),
        }
    }
    pub fn run(&mut self) {
        println!("getting items from channel over 2 second");
        let start = std::time::Instant::now();
        while start.elapsed().as_secs() < 3 {
            let item = self.receiver.try_recv();
            match item {
                Ok(item) => {
                    println!("reciving: {}", item);
                    if item == 150 {
                        println!("done");
                        break;
                    }
                },
                Err(err) => match err {
                    std::sync::mpsc::TryRecvError::Empty => {
                        println!("reciving None");
                        thread::sleep(std::time::Duration::from_millis(2));
                    }
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        println!("channel disconnected");
                    }
                },
            }
        }
    }
    fn start_producer() -> Receiver<i32> {
        let (sender, receiver) = channel::<i32>();
        std::thread::spawn(move || {
            let mut i = 0;
            while let Ok(_) = sender.send(i) {
                println!("sending: {}", i);
                i += 1;
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });
        receiver
    }
}

pub fn run() {
    let (sender, receiver) = channel::<i32>();
    std::thread::spawn(move || {
        let mut i = 0;
        while let Ok(_) = sender.send(i) {
            println!("sending: {}", i);
            i += 1;
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });
    println!("getting items from channel over 2 second");
    let start = std::time::Instant::now();
    while start.elapsed().as_secs() < 3 {
        let item = receiver.try_recv();
        match item {
            Ok(item) => {
                println!("reciving: {}", item);
                if item == 150 {
                    println!("done");
                    break;
                }
            },
            Err(err) => match err {
                std::sync::mpsc::TryRecvError::Empty => {
                    println!("reciving None");
                    thread::sleep(std::time::Duration::from_millis(3));
                }
                std::sync::mpsc::TryRecvError::Disconnected => {
                    println!("channel disconnected");
                }
            },
        }
    }
}
pub fn run2() {
    let (sender, receiver) = channel::<i32>();
    std::thread::spawn(move || {
        let mut i = 0;
        loop {
            if sender.send(i).is_err() {
                break;
            }
            println!("sending: {}", i);
            i += 1;
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });
    println!("getting items from channel over 2 second");
    let start = std::time::Instant::now();
    while start.elapsed().as_secs() < 3 {
        let item = receiver.try_recv();
        match item {
            Ok(item) => {
                println!("reciving: {}", item);
                if item == 150 {
                    println!("done");
                    break;
                }
            },
            Err(err) => match err {
                std::sync::mpsc::TryRecvError::Empty => {
                    println!("reciving None");
                    thread::sleep(std::time::Duration::from_millis(3));
                }
                std::sync::mpsc::TryRecvError::Disconnected => {
                    println!("channel disconnected");
                }
            },
        }
    }
}

