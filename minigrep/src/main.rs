extern crate minigrep;

use std::env;
use std::process;

use minigrep::*;

fn main() {
	let argvec: Vec<String> = env::args().collect();

	let config = Config::new(&argvec).unwrap_or_else(|err| {
		eprintln!("Problem parsing args: {}", err);
		process::exit(1);
	});

	match run(config) {
		Ok(()) => (),
		Err(e) => eprintln!("Recieved error: {}", e)
	};
}
