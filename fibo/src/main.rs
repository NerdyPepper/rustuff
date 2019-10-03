fn main() {
    let n = 8;
    let mut memo = vec![];
    let mut f: u32;
    for k in 1..n + 1 {
        if k <= 2 {
            f = 1;
        } else {
            f = memo[k - 2] + memo[k - 3];
        }
        memo.push(f);
    }
    println!("{:?}", memo);
}
