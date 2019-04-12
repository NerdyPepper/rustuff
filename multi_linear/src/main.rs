use std::thread;

fn main() {
    let search_element: i32 = 0;
    let mut found: (bool, i32) = (false, 0);
    const MAX: i32 = 1024;
    let no_of_threads = 4;
    let mut handles = vec![];

    for i in 0..no_of_threads {
        handles.push(thread::spawn(move || {
            for x in i*MAX/no_of_threads .. (i+1)*MAX/no_of_threads {
                if x == search_element {
                    found = (true, i);
                }
            }
        }));
    }

    for handle in handles { handle.join().unwrap() };
    if found.0 {
        println!("{:?}", found)
    } else {
        println!("Not found!");
    }
}
