extern crate regex;
use regex::Regex;

fn main() {
    let re1 = Regex::new(r"^== (?P<d>.*) ==$").unwrap();
    let re2 = Regex::new(r"^=== (?P<d>.*) ===$").unwrap();

    let before1 = "== Title ==";
    let before2 = "=== Title ===";

    let after1 = re1.replace(before1, "\x1b[1m$d\x1b[0m");
    let after2 = re2.replace(before2, "\x1b[5m$d\x1b[0m");

    println!("{}\n{}", after1, after2);
}
