fn main() {
    let num_list = vec![123, 123, 435, 12345];
    let char_list = vec!['a', 'b', 'c'];

    println!("The largest number is {}", largest(&num_list));
    println!("The largest char is {}", largest(&char_list));
}

fn largest<T: std::cmp::PartialOrd + std::marker::Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}
