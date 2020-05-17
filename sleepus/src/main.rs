use std::thread::{ sleep, spawn };
use std::time::Duration;

fn sleepus() {
    for i in 1..=10 {
        println!("sleepus {}", i);
        sleep(Duration::from_millis(500));
    }
}

fn interruptus() {
    for i in 1..=5 {
        println!("interruptus {}", i);
        sleep(Duration::from_millis(1000));
    }
}

fn main() {
    // let sleepus = spawn(sleepus);
    // let interruptus = spawn(interruptus);

    // sleepus.join().unwrap();
    // interruptus.join().unwrap();
    let sleepus = spawn(sleepus);
    interruptus();

    sleepus.join().unwrap();
}
