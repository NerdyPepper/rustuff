use std::net::ToSocketAddrs;
use url::{Url, Host};

fn main() {
    let test = Url::parse("http://3468664375@3468664375/o%62s%63ur%65%2e%68t%6D").unwrap();

    println!("{:?}", test.scheme());
    println!("{:?}", test.username());
    println!("{:?}", test.password());
    println!("{:?}", test.host_str());

    let test_hex = "obscure.htm".as_bytes();
    for letter in test_hex {
        print!("%");
        print!("{:x}", letter);
    }
}
