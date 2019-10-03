use num_bigint::{BigInt, ToBigInt};

fn main() {
    let big: BigInt = 999999999.to_bigint().unwrap()
        * 999999999.to_bigint().unwrap()
        * 99999999.to_bigint().unwrap();
    let big2: BigInt = 2.to_bigint().unwrap();
    if 9999999999999f64 > std::f64::MAX {
        println!("BIG");
    }
    println!("{}", big);
}
