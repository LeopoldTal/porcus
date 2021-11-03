use porcus;
use std::io::{self, Write};

fn main() {
	let transformer = porcus::get_default_transformer();

	loop {
		let mut input = String::new();
		let read_size = io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if read_size == 0 {
			break;
		}

		let pig_latin = transformer.to_pig_latin(input);
		if let Err(_) = io::stdout().write(pig_latin.as_bytes()) {
			break;
		}
	}

	io::stdout().flush().expect("Failed to flush stdout buffer");
}
