fn main() {
    println!("Hello, world!");
    let mut myvec: Vec<i32> = Vec::new();

    myvec.push(12);
    myvec.push(11);
    myvec.push(13);
    myvec.push(14);
    myvec.pop();

    println!("{:#?}", myvec);
}
