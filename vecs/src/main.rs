fn main() {
    println!("Hello, world!");
    let mut myvec: Vec<i32> = Vec::new();

    myvec.push(12);
    myvec.push(11);
    myvec.push(13);
    myvec.push(14);

    let third: Option<&i32> = myvec.get(2);

    println!("{:?}", third);

    for i in &myvec {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum MyVal {
        Intz(i32),
        Float(f64),
        Text(String),
    }

    let valvec = vec![
        MyVal::Intz(2),
        MyVal::Float(3.0),
        MyVal::Text(String::from("hi guys")),
    ];

    let valvectest = &valvec[1];

    println!("{:?}", valvectest);

    println!("{:?}", valvec.get(1));
}
