fn main() {
    let mut hello = "hello";
    let new_hello = &hello.replace("h", "a");
    println!("{}", new_hello);
}

