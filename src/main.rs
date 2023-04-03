use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::net::tcp::ReadHalf;
use tokio::net::tcp::WriteHalf;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::io::AsyncBufReadExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

#[tokio::main]
async fn main() {
    println!("chat-tokio-stream");

    let listener: TcpListener = TcpListener::bind("localhost:8484").await.unwrap();

    let (tx, _rx): (Sender<String>, Receiver<String>) = broadcast::channel::<String>(10);

    loop {
        let (mut socket, _addr): (TcpStream, SocketAddr) = listener.accept().await.unwrap();
        let tx: Sender<String>  = tx.clone();
        let mut rx: Receiver<String> = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer): (ReadHalf, WriteHalf) = socket.split();

            let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
            let mut line: String = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        let _no_of_subscriptions: usize = tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let msg: String = result.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
