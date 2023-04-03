use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::net::tcp::ReadHalf;
use tokio::net::tcp::WriteHalf;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() {
    println!("chat-tokio-stream");

    let listener: TcpListener = TcpListener::bind("localhost:8484").await.unwrap();

    loop {
        let (mut socket, _addr): (TcpStream, SocketAddr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer): (ReadHalf, WriteHalf) = socket.split();

            let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
            let mut line: String = String::new();

            loop {
                let bytes_read: usize = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }

                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
