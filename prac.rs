use std::convert::From;
use std::string::ToString;

#[derive(Debug)]
struct Noomba {
    value: i32,
}

impl From<i32> for Noomba {
    fn from(item: i32) -> Self {
        Noomba { value: item }
    }
}

// impl fmt::Display for Noomba {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Value {}", self.value)
//     }
// }

impl ToString for Noomba {
    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}

fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    let mut n = 1;
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
