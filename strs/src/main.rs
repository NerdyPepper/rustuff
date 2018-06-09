fn main() {
    say_hello();

    let mut hello = String::from("hello ");
    let world = "world!";
    hello.push_str(world);

    println!("{}", hello);
}

fn say_hello() {
    let hello = String::from("Dobrý den");
    println!("{}", hello);
}
