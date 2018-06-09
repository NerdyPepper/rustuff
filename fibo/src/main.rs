fn main() {
    let val: i32 = 19;

    fibo_calc(val);
}

fn fibo_calc(x: i32) {
    let mut index = 0;

    let mut prev = 1;
    let mut current = 1;
    let mut next = prev + current;

    println!("{}", prev);
    println!("{}", current);

    while index < x-1 {
        println!("{}", next);

        prev = current;
        current = next;
        next = prev + current;

        index = index + 1;
    }
}
