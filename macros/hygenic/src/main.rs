#[macro_use]
#[feature(trace_macros)]

// macro_rules! foo {
//     () => {
//         let x = 0;
//     }
// }

//macro_rules! foo2 {
//    ($x: ident) => {
//        $x = 0;
//    }
//}

fn main() {
    let mut x = 42;

    let mut x = 32;

    macro_rules! foo {
        () => {
            x = 0;
        }
    }

    trace_macros!(true);
    foo!();
    trace_macros!(false);

    println!("{}", x);
}
