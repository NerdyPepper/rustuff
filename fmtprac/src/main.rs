use std::fmt;

fn main() {
    let bangalore = City {
        lat: 25.2525,
        lon: -35.3535,
    };

    let blue = Color {
        red: 0,
        green: 25,
        blue: 200,
    };

    println!("Bangalore: {}", bangalore);
    println!("Blue: {}", blue);
}

struct City {
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat > 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon > 0.0 { 'E' } else { 'W' };
        write!(f, "{:.3} {}, {:.3} {}",
               self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#");
        write!(f, "{:02x}", self.red);
        write!(f, "{:02x}", self.green);
        write!(f, "{:02x}", self.blue)
    }
}
