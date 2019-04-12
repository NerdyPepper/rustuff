use std::rc::Rc;

fn main() {
    let x = Rc::new(5);

    let _y = Rc::clone(&x);
    let _z = Rc::clone(&x);
    let _a = Rc::clone(&x);
    let _b = Rc::clone(&x);
    let _c = Rc::clone(&x);

    println!("{}", Rc::strong_count(&x));
}
