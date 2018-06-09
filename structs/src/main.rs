use std::fmt;

fn main() {

	let mydoggo = Dogchan {
		dogname: String::from("Blaggie"),
		nick: String::from("B"),
		age: 3,
		good: true,
	};
	println!("Dogname: {}", mydoggo.dogname);

	let frendoggo = adopt_dog("Whitey".to_string(), "W".to_string(), 3);
	println!("Dogname: {}", frendoggo.dogname);

	println!("Nice la {:#?}", mydoggo);
	println!("{}", mydoggo);
}

#[derive(Debug)]
struct Dogchan {
	dogname: String,
	nick: String,
	age: i32,
	good: bool,
}

impl fmt::Display for Dogchan {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Name's {}", self.dogname);
		write!(f, ", aged {}", self.age);
		write!(f, ", a.k.a {}", self.nick)
	}
}

fn adopt_dog(dogname: String, nick: String, age: i32) -> Dogchan {
	Dogchan {
		nick,
		dogname,
		age,
		good: true, // all dogs are good
	}
}
