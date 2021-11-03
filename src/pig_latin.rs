use crate::case;
use crate::{get_char_type_at, CharType};
use unicode_script::UnicodeScript;
use unicode_segmentation::UnicodeSegmentation;

pub struct PigLatinTransformer {
	pub consonant_suffix: String,
	pub vowel_suffix: String,
}

pub fn get_default_transformer() -> PigLatinTransformer {
	PigLatinTransformer {
		consonant_suffix: String::from("ay"),
		vowel_suffix: String::from("way"),
	}
}

impl PigLatinTransformer {
	pub fn to_pig_latin(&self, s: String) -> String {
		s.split_word_bounds()
			.map(|word| self.word_to_case_matched_pig_latin(word))
			.collect::<Vec<String>>()
			.concat()
	}

	fn word_to_case_matched_pig_latin(&self, s: &str) -> String {
		if should_skip_word(s) {
			return s.to_string();
		}

		let pig = self.word_to_uncased_pig_latin(s);
		case::to_case(pig, case::detect_case(s))
	}

	fn word_to_uncased_pig_latin(&self, s: &str) -> String {
		let graphemes = &s.graphemes(true).collect::<Vec<&str>>();

		let mut prefix_length = 0;
		while has_consonant_at(graphemes, prefix_length) {
			prefix_length += 1;
		}

		if prefix_length == 0 {
			return format!("{}{}", s, self.vowel_suffix);
		}
		let prefix = &graphemes[0..prefix_length];
		let suffix = &graphemes[prefix_length..];
		format!(
			"{}{}{}",
			suffix.concat(),
			prefix.concat(),
			self.consonant_suffix
		)
	}
}

fn should_skip_word(s: &str) -> bool {
	if let Some(first_char) = s.chars().next() {
		first_char.script().full_name() != "Latin"
	} else {
		true
	}
}

fn has_consonant_at(graphemes: &Vec<&str>, index: usize) -> bool {
	match get_char_type_at(graphemes, index) {
		CharType::Consonant => true,
		CharType::Ambiguous => matches!(get_char_type_at(graphemes, index + 1), CharType::Vowel),
		_ => false,
	}
}

#[cfg(test)]
mod test_default_pig_latin {
	use super::*;

	fn assert_pig_latin(input: &str, expected: &str) {
		let transformer = get_default_transformer();
		assert_eq!(transformer.to_pig_latin(String::from(input)), expected);
	}

	#[test]
	fn single_word() {
		assert_pig_latin("nix", "ixnay");
		assert_pig_latin("scram", "amscray");
		assert_pig_latin("string", "ingstray");
		assert_pig_latin("joy", "oyjay");
		assert_pig_latin("oy", "oyway");
		assert_pig_latin("aid", "aidway");
		assert_pig_latin("hmm", "hmmay");
	}

	#[test]
	fn y_as_consonant() {
		assert_pig_latin("yoga", "ogayay");
		assert_pig_latin("Yiddish", "Iddishyay");
	}

	#[test]
	fn y_as_vowel() {
		assert_pig_latin("ytterbium", "ytterbiumway");
		assert_pig_latin("Ypres", "Ypresway");
		assert_pig_latin("Yvonne", "Yvonneway");
		assert_pig_latin("yyadzehe", "yyadzeheway");
		assert_pig_latin("yy", "yyway");
	}

	#[test]
	fn diacritics() {
		assert_pig_latin("café", "afécay");
		assert_pig_latin("ça", "açay");
		assert_pig_latin("çà", "àçay");
		assert_pig_latin("âge", "âgeway");
		assert_pig_latin("Éole", "Éoleway");
		assert_pig_latin("Česko", "Eskočay");
		assert_pig_latin("článek", "ánekčlay");
		assert_pig_latin("Słowacją", "Owacjąsłay");
		assert_pig_latin("ščepec", "epecščay");
	}

	#[test]
	fn latin_supplement() {
		assert_pig_latin("œuf", "œufway");
		assert_pig_latin("sœur", "œursay");
		assert_pig_latin("ﬀion", "ionﬀay");
		assert_pig_latin("ʁɛv", "ɛvʁay");
	}

	#[test]
	fn not_latin() {
		assert_pig_latin("", "");
		assert_pig_latin("दिखना", "दिखना");
		assert_pig_latin("twerkना", "erkनाtway");
		// not sure about these, could change my mind
		assert_pig_latin("αGo", "αGo");
		assert_pig_latin("TV9मराठी", "9मराठीTVAY");
	}

	#[test]
	fn case() {
		assert_pig_latin("hello", "ellohay");
		assert_pig_latin("Hello", "Ellohay");
		assert_pig_latin("HELLO", "ELLOHAY");
		assert_pig_latin("heLLo", "eLLohay");
		assert_pig_latin("iPhone", "iPhoneway");
		assert_pig_latin("EGG", "EGGWAY");
	}

	#[test]
	fn sentence() {
		assert_pig_latin("hello world", "ellohay orldway");
		assert_pig_latin("hello-hi", "ellohay-ihay");
		assert_pig_latin("Yes (no)", "Esyay (onay)");
		assert_pig_latin("Hello, ADORABLE world!", "Ellohay, ADORABLEWAY orldway!");
		assert_pig_latin("🦀 My name is मनीष. 📎", "🦀 Ymay amenay isway मनीष. 📎");
		assert_pig_latin("L'eau d'orange", "Eaul'ay oranged'ay");
		assert_pig_latin("P'sst ! Par ici !", "P'sstay ! Arpay iciway !");
		assert_pig_latin("Simon Example z״l", "Imonsay Exampleway z״lay");
		assert_pig_latin("Ploni Almoni a״h", "Oniplay Almoniway a״hway");
		assert_pig_latin("The Rebbe z״ya", "Ethay Ebberay az״yay");
	}

	#[test]
	fn custom_suffixes() {
		let transformer = PigLatinTransformer {
			consonant_suffix: String::from("yay"),
			vowel_suffix: String::from("-hay"),
		};
		let result = transformer.to_pig_latin(String::from("Hello, egg!"));

		assert_eq!(result, "Ellohyay, egg-hay!");
	}
}
