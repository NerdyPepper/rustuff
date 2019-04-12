fn main() {
    let a: Vec<i32> = vec![1, 2, 3, 4];

    let double: Vec<i32> = a.iter()
        .map(|x| x * x)
        .collect();

    println!("{:?}", double);
}
