use std::ops::Deref;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

#[derive(Debug)]
struct MyBox<T>{
    x: T
}

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox{
            x: x
        }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.x
    }
}

impl<T> Drop for MyBox<T> {
    fn drop (&mut self) {
        println!("Dropping the data");
    }
}

fn main() {
    let mut x = 5;
    let y = Box::new(x);
    x += 5;
    println!("y = {}", *y);
    println!("x = {}", x);
}

