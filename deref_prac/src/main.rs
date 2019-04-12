use std::ops::Deref;

#[derive(Debug)]
struct SquareBox {
    x: i32,
    y: i32,
}

impl SquareBox {
    fn new(x: i32) -> SquareBox {
        SquareBox {
            x: x,
            y: x * x,
        }
    }
}

impl Deref for SquareBox {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.y
    }
}

fn main() {
    let one = 5;
    let two = SquareBox::new(one);

    println!("{}", *two);
}
