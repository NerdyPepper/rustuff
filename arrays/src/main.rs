fn main() {
    let mut arr1: [i32; 5] = [1, 2, 3, 4, 5];

    for item in arr1.iter() {
        print!("{},", item);
    }

    analyze_slice(&mut arr1[1..3]);

    for item in arr1.iter() {
        print!("{},", item);
    }
}

fn analyze_slice(slice: &mut [i32]) {
    println!("\n first element: {}", slice[0]);
    println!("slice has {} elems", slice.len());
}
