fn main() {
    let hai = String::from("Hello world");
    let first = first_word(&hai);

    println!("This is the first word {}", first);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
