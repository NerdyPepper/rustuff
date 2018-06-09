#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };

    let rect2 = Rect {
        width: 200,
        height: 10,
    };

    println!(
        "Area is {}",
        rect.area()
        );

    println!( 
        "Can rect hold rect2?: {}",
        rect.can_hold(&rect2)
        );

}
