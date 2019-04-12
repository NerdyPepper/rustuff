trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Square {
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(figure: T) {
    println!("Area of fig is {}", figure.area());
}

fn main() {
    println!("Hello, world!");
    
    let c = Circle {
        radius: 10.0f64
    };

    let s = Square {
        side: 5.0f64
    };

    print_area(c);
    print_area(s);
}
