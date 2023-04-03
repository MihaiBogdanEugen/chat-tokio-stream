use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    println!("chat-tokio-stream");

    let listener: TcpListener = TcpListener::bind("localhost:8484").await.unwrap();
    let (mut socket, _addr): (TcpStream, SocketAddr) = listener.accept().await.unwrap();

    loop {
        let mut buffer: [u8; 1024] = [0u8; 1024];  //1KB
        let bytes_read: usize = socket.read(&mut buffer).await.unwrap();
        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}
