fn main() {
    println!("Hello, world!");
}

struct Cacher<T>
where T: Fn(u32) -> u32
{
    calc: T,
    val: Option<u32>,
}

impl<T> Cacher<T> {
    fn new(calc: T) -> Cacher<T> {
        Cacher{
            calc,
            val: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.val {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.val = Some(v);
                v
            },
        }
    }
}
