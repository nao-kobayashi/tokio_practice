use tokio::{io, task, time};
use tokio::process::Command;
use std::time::Duration;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let date = task::spawn(dating());
    let cp = task::spawn(copying());

    date.await??;
    cp.await??;
    Ok(())
}

async fn dating() -> Result<(), std::io::Error> {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        Command::new("date").spawn()?.await?;
    }
}

async fn copying() -> Result<(), std::io::Error> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    io::copy(&mut stdin, &mut stdout).await?;
    Ok(())
}



// use tokio::task;
// use tokio::time;
// use tokio::process::Command;
// use std::time::Duration;
// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     task::spawn(dating()).await??;
//     Ok(())
// }

// async fn dating() -> Result<(), std::io::Error> {
//     let mut interval = time::interval(Duration::from_secs(1));
//     loop {
//         interval.tick().await;
//         Command::new("date").spawn()?.await?;
//     }
// }




// use tokio::time;
// use tokio::process::Command;
// use std::time::Duration;
// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let mut interval = time::interval(Duration::from_secs(1));
//     loop {
//         interval.tick().await;
//         Command::new("date").spawn()?.await?;
//     }
// }


// use tokio::io::{self, AsyncWriteExt};
// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let mut stdout = io::stdout();
//     stdout.write_all(b"Hello, world!\n").await?;
//     Ok(())
// }




// use tokio::io;
// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let mut stdout = io::stdout();
//     let mut hello: &[u8] = b"Hello World!\n";
//     io::copy(&mut hello, &mut stdout).await?;
//     Ok(())
// }
