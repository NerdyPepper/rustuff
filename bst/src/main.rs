use Tree::{ Item, Nil };
fn main() {
    println!("Hello, world!");
}

enum Tree {
    Item(u32, Box<Tree>, Box<Tree>),
    Nil,
}

impl Tree {
    fn new() -> Tree {
        Nil
    }
    fn insert(&mut self, elem: u32) {
        match *self {
            Item(parent, left, right) => {
                if elem < parent {

                }
            }
        }
    }
}
