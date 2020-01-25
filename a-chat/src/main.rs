use async_std::{
    net::{TcpListener, ToSocketAddrs, TcpStream},
    prelude::*,
    io::BufReader,
    task,
};

use futures::channel::mpsc;
use futures::sink::SinkExt;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

fn log_error<F>(fut: F) -> task::JoinHandle<()> 
where F: Future<Output = Result<()>> + Send + 'static
{
    task::spawn(async move {
        if let Err(r) = fut.await {
            eprintln!("{:?}", r);
        }
    })
}

async fn connection_writer_loop(
    mut messages: Receiver<String>,
    stream: Arc<TcpStream>
) -> Result<()> {
    let mut stream = &*stream;
    while let Some(msg) = messages.next().await {
        stream.write_all(msg.as_bytes()).await?;
    }
    Ok(())
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        // TODO
        let stream = stream?;
        println!("Accepting stream from - {}", stream.peer_addr()?);
        let _handle = task::spawn(connection_loop(stream));
        log_error(_handle);
    }
    Ok(())
}


async fn connection_loop(s: TcpStream) -> Result<()> {
    let reader = BufReader::new(&s);
    let mut lines = reader.lines();

    let name = match lines.next().await {
        None => Err("client disconnected immidiately!")?,
        Some(l) => l?,
    };

    println!("name: {}", name);

    while let Some(l) = lines.next().await {
        let l = l?;
        let (dest, msg) = match l.find(':') {
            None => continue,
            Some(idx) => (&l[..idx], l[idx + 1 ..].trim()),
        };
        let dest: Vec<String> = dest.split(',')
            .map(|name| name.trim().to_string())
            .collect();
        let msg: String = msg.to_string();
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let fut = accept_loop("127.0.0.1:7878");
    task::block_on(fut)
}
