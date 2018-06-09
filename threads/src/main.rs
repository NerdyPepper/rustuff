use std::thread;
use std::time::Duration;

fn main() {
    let a: f64 = 163877354725619.0;
    let b: f64 = 30051054552980641676923138.0;

    let mut s: f64 = b / a;

    let handle = thread::spawn(|| {
        for d in 1..273645183562i128 {
            let d = d as f64;
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}
