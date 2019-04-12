use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

fn main() {
    let f = File::open("test.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines {

    }
}
