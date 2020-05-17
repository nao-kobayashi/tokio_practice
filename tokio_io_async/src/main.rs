use tokio::prelude::*;
use tokio::io::AsyncBufReadExt;
use std::sync::Arc;

use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    let _ = args.next();
    let mut tasks = vec![];
    let count = Arc::new(Mutex::new(0_u32));

    for filename in args {
        let count = count.clone();
        tasks.push(
            tokio::spawn(async move {
                let file = tokio::fs::File::open(filename).await?;
                let file = io::BufReader::new(file);
                let mut lines = file.lines();
                let mut local_count = 0u32;
                while let Some(_) = lines.next_line().await? {
                    local_count += 1;
                }

                let mut count = count.lock().await;
                *count += local_count;
                Ok(()) as Result<(), std::io::Error>
            })
        );
    }

    for task in tasks {
        task.await??;
    }

    let count = count.lock().await;
    println!("total lines: {}", *count);
    Ok(())
}
