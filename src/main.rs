use clap::{App, Arg};
use porcus::{PigLatinTransformer, DEFAULT_CONSONANT_SUFFIX, DEFAULT_VOWEL_SUFFIX};
use std::io::{self, Write};

fn main() {
	let matches = App::new("porcus")
		.version("0.1.0")
		.about("Transforms standard input to pig latin")
		.arg(
			Arg::with_name("consonant_suffix")
				.short("c")
				.long("consonant")
				.default_value(DEFAULT_CONSONANT_SUFFIX)
				.help("suffix for words starting with a consonant"),
		)
		.arg(
			Arg::with_name("vowel_suffix")
				.short("v")
				.long("vowel")
				.default_value(DEFAULT_VOWEL_SUFFIX)
				.help("suffix for words starting with a vowel"),
		)
		.get_matches();

	let consonant_suffix = matches
		.value_of("consonant_suffix")
		.expect("Consonant suffix not found in args");
	let vowel_suffix = matches
		.value_of("vowel_suffix")
		.expect("Vowel suffix not found in args");

	let transformer = PigLatinTransformer::new(consonant_suffix, vowel_suffix);

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
