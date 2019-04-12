use std::collections::HashMap;

#[macro_use]
macro_rules! hello {
    () => { println!("Hello world!"); }
}

macro_rules! yo {
    ($name:expr) => { println!("{}", $name); }
}

macro_rules! hey {
    ($name:expr) => {
        println!("{}", $name);
    };
    ($( $name:expr ),*) => {
        $(println!("{}", $name);)*
    };
}

macro_rules! my_print {
    (foo <> $e: expr) => {
        println!("Fooo!! {}", $e);
    };
    ($e: expr) => {
        println!("{}", $e);
    };
    ($i: ident, $e: expr) => {
        let $i = {
            let a = $e;
            println!("{}", a);
            a
        };
    }
}

macro_rules! map {
    ($( $key:expr => $value:expr ),*) => {{
        let mut hm = HashMap::new();
        $( hm.insert($key, $value); )*
            hm
    }};
}

fn main() {
    hello!();
    yo!("akshay, yo");
    hey!("hi1", "hi2", "hi3");
    my_print!(x, 30 + 10);
    my_print!(10 + 5);
    my_print!(foo <> "hello");

    let hash = map!(
        "name" => "akshay",
        "age" => "18"
    );

    println!("{:?}", hash);
}
