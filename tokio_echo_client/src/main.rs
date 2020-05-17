use tokio::io;
use tokio::task::spawn;
use tokio::net::{ TcpStream };

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut recv, mut send) = io::split(stream);
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // error borrowed value does not live long enough
    // let send = spawn(io::copy(&mut stdin, &mut send));
    // let recv = spawn(io::copy(&mut recv, &mut stdout));

    let send = spawn(async move {
        io::copy(&mut stdin, &mut send).await
    });

    let recv = spawn(async move {
        io::copy(&mut recv, &mut stdout).await
    });

    send.await??;
    recv.await??;
    Ok(())
}