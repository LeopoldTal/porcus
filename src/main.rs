use porcus::PigLatinTransformer;
use std::io::{self, Write};

fn main() {
	// TODO:
	// - show help
	// - pass suffixes on the command line
	let transformer = PigLatinTransformer::default();

	loop {
		let mut input = String::new();
		let read_size = io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if read_size == 0 {
			break;
		}

		let pig_latin = transformer.to_pig_latin(input);
		if io::stdout().write(pig_latin.as_bytes()).is_err() {
			break;
		}
	}

	io::stdout().flush().expect("Failed to flush stdout buffer");
}
