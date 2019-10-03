#![allow(dead_code)]
#![allow(unused_imports)]

use std::sync::{ RwLock, Arc };
use std::thread;

fn main() {
    let datum = Arc::new(RwLock::new(10));

    let readers: Vec<_> = (0..2).map(|_| {
        let datum = Arc::clone(&datum);
        thread::spawn( move || {
            let reader = datum.read().unwrap();
            println!("Read {}", reader);
        })
    }).collect();

    let write_clone = Arc::clone(&datum);
    let writer = thread::spawn( move || {
        let mut w = write_clone.write().unwrap();
        *w = 11;
    });

    for r in readers {
        r.join().unwrap();
    }
    writer.join().unwrap();
}

