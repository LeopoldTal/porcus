use porcus;
use std::io;

fn main() {
	loop {
		let mut input = String::new();
		let read_size = io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if read_size == 0 {
			break;
		}

		let as_pig_latin = porcus::to_pig_latin(input);
		print!("{}", as_pig_latin);
	}
}
