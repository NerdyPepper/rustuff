
use std::net::{TcpStream};
use std::io::{Read, Write};

fn main() {
    match TcpStream::connect("localhost:3000") {
        Ok(mut stream) => {
            println!("Connection success!");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent {}, awaiting reply...", String::from_utf8_lossy(msg));

            let mut data = [0 as u8; 6];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    println!("Reply is {}", String::from_utf8_lossy(&data));
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
