#![allow(dead_code)]
#![allow(unused_imports)]

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
    where F: Fn(&(u8, u16)) -> &u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn do_it(data: &(u8, u16)) -> &u8 { &data.0 }

use num::Num;

fn main() {
    // let result = <f64 as Num>::from_str_radix("24.5", 2);
    // println!("{:?}", result)
    
    println!("{}", std::u32::MAX);
}
