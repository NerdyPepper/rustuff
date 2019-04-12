
fn main() {
    let mut arr = [[0i32; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            if i >= j {
                arr[i][j] = 1;
            }
            print!("{} ", arr[i][j]);
        }
        println!("");
    }

    println!("");
    println!("{:?}", arr);
}
