use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(0i32));
    let (tx, rx) = channel::<i32>();

    for _ in 0..10 {
        let (r, t) = (Arc::clone(&data), tx.clone());
        thread::spawn(move || {
            let mut inner_data = r.lock().unwrap();

            *inner_data += 1;
            t.send(*inner_data).unwrap();
        });
    }

    while let Ok(x) = rx.recv_timeout(Duration::from_secs(1)) {
        println!("{}", x);
    }
}
