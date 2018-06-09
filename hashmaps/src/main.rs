use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 10);

    println!("{:?}", scores);

    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Green"), 20);


    scores.entry(String::from("Red")).or_insert(20);
    scores.entry(String::from("Yellow")).or_insert(20);

    println!("{:?}", scores);

    let text = "this is very nice this";
    let mut wordmap = HashMap::new();

    for word in text.split_whitespace() {
        let count = wordmap.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", wordmap);
}

// fn print_hash(x: HashMap) {
//     println!("{:?}", x);
// }
