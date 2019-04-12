use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use std::sync::{ Mutex, Arc };

fn main() {
    let counter = Arc::new( Mutex::new(0) );
    let mut handles = vec![];

    for _ in 0..10 {
        let count = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut k = count.lock().unwrap();
            *k += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let j = counter.lock().unwrap();
    println!("{}", *j);
}
