use async_std::task::{ sleep, spawn };
use std::time::Duration;

async fn sleepus() {
    for i in 1..=10 {
        println!("sleepus {}", i);
        sleep(Duration::from_millis(500)).await;
    }
}

fn sleepus2() -> impl std::future::Future<Output=()> {
    async {
        for i in 1..=10 {
            println!("sleepus async block {}", i);
            sleep(Duration::from_millis(300)).await;
        }
    }
}

async fn interruptus() {
    for i in 1..=5 {
        println!("interruptus {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}

#[async_std::main]
async fn main() {
    let sleepus = spawn(sleepus());
    let interruprus = spawn(interruptus());
    sleepus2().await;

    interruprus.await;
    sleepus.await;
}
