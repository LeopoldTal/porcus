use super::{DEFAULT_CONSONANT_SUFFIX, DEFAULT_VOWEL_SUFFIX};
use crate::case;
use crate::char_type::{self, CharType};
use std::fmt;
use unicode_script::UnicodeScript;
use unicode_segmentation::UnicodeSegmentation;

/// Converter to pig latin.
///
/// # Definition
///
/// [Pig latin](https://en.wikipedia.org/wiki/Pig_Latin) is a transformation applied to each word
/// within a text.
///
/// - If a word starts with a consonant, all initial consonants are moved to the end of the word,
/// then the suffix "ay" is appended, e.g. `nix` becoomes `ixnay` and `scram` becomes `amscray`.
/// - If it starts with a vowel, the suffix "way" is appended to it, e.g. `eat` becomes `eatway`.
/// There are many variants of pig latin using different vowel suffixes, such as "yay" or "hay".
///
/// # Examples
///
/// By default, "ay" is appended to words starting with a consonant, and "way" to those starting
/// with a vowel.
///
/// ```
/// # use porcus::PigLatinTransformer;
/// let transformer = PigLatinTransformer::default();
/// let pig_latin = transformer.to_pig_latin("Hi all!");
/// assert_eq!(pig_latin, "Ihay allway!");
/// ```
///
/// The letter "Y" and its variants are treated as either vowels or consonants depending on the
/// following letter.
///
/// ```
/// # use porcus::PigLatinTransformer;
/// # let transformer = PigLatinTransformer::default();
/// assert_eq!(transformer.to_pig_latin("Vas-y !"), "Asvay-yway !");
/// assert_eq!(transformer.to_pig_latin("Yvonne"), "Yvonneway");
/// assert_eq!(transformer.to_pig_latin("yak"), "akyay");
/// assert_eq!(transformer.to_pig_latin("ýy"), "ýyway");
/// assert_eq!(transformer.to_pig_latin("byrå"), "yråbay");
/// ```
///
/// All Latin-script letters are supported, including combining diacritics and IPA extensions.
///
/// ```
/// # use porcus::PigLatinTransformer;
/// # let transformer = PigLatinTransformer::default();
/// assert_eq!(transformer.to_pig_latin("grüßt"), "üßtgray");
/// assert_eq!(transformer.to_pig_latin("pɪɡ lætɪn"), "ɪɡpay ætɪnlay");
/// ```
///
/// Only words starting with Latin characters are transformed.
///
/// ```
/// # use porcus::PigLatinTransformer;
/// # let transformer = PigLatinTransformer::default();
/// assert_eq!(transformer.to_pig_latin("TV9मराठी"), "9मराठीTVAY");
/// assert_eq!(transformer.to_pig_latin("42 µm"), "42 µm");
/// assert_eq!(transformer.to_pig_latin("Chinese / 中文"), "Inesechay / 中文");
/// ```
///
/// You can also supply your own suffixes for consonants and vowels.
/// Pig latin almost always uses "ay", though "-ay" is also found. For vowels, many different
/// suffixes are popular: "way", "yay", "tay", "hay", "-ay", "-hay".
///
/// ```
/// # use porcus::PigLatinTransformer;
/// let transformer = PigLatinTransformer::new("-ay", "-yay");
/// let pig_latin = transformer.to_pig_latin("Hi all!");
/// assert_eq!(pig_latin, "Ih-ay all-yay!");
/// ```
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PigLatinTransformer {
	consonant_suffix: String,
	vowel_suffix: String,
}

impl fmt::Display for PigLatinTransformer {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Pig Latin <C+{} V+{}>",
			self.consonant_suffix, self.vowel_suffix
		)
	}
}

impl Default for PigLatinTransformer {
	fn default() -> Self {
		Self {
			consonant_suffix: String::from(DEFAULT_CONSONANT_SUFFIX),
			vowel_suffix: String::from(DEFAULT_VOWEL_SUFFIX),
		}
	}
}

impl PigLatinTransformer {
	pub fn new<Sc, Sv>(consonant_suffix: Sc, vowel_suffix: Sv) -> Self
	where
		Sc: Into<String>,
		Sv: Into<String>,
	{
		Self {
			consonant_suffix: consonant_suffix.into(),
			vowel_suffix: vowel_suffix.into(),
		}
	}

	/// Gets the suffix appended to words starting with a consonant.
	#[must_use]
	pub const fn consonant_suffix(&self) -> &String {
		&self.consonant_suffix
	}
	/// Gets the suffix appended to words starting with a vowel.
	#[must_use]
	pub const fn vowel_suffix(&self) -> &String {
		&self.vowel_suffix
	}

	/// Returns the pig latin translation of a string.
	pub fn to_pig_latin<S: Into<String>>(&self, s: S) -> String {
		s.into()
			.split_word_bounds()
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
	s.chars().next().map_or(true, |first_char| {
		first_char.script().full_name() != "Latin"
	})
}

fn has_consonant_at(graphemes: &[&str], index: usize) -> bool {
	match char_type::get_char_type_at(graphemes, index) {
		CharType::Consonant => true,
		CharType::Ambiguous => matches!(
			char_type::get_char_type_at(graphemes, index + 1),
			CharType::Vowel
		),
		_ => false,
	}
}

#[cfg(test)]
mod test_getters {
	use super::*;

	#[test]
	fn suffixes() {
		let transformer = PigLatinTransformer::new("C", "V");
		assert_eq!(transformer.consonant_suffix(), "C");
		assert_eq!(transformer.vowel_suffix(), "V");

		let transformer = PigLatinTransformer::default();
		assert_eq!(transformer.consonant_suffix(), "ay");
		assert_eq!(transformer.vowel_suffix(), "way");
	}
}

#[cfg(test)]
mod test_to_pig_latin {
	use super::*;

	fn assert_pig_latin(input: &str, expected: &str) {
		let transformer = PigLatinTransformer::default();
		assert_eq!(transformer.to_pig_latin(input), expected);
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
		assert_pig_latin("I", "Iway");
	}

	#[test]
	fn sentence() {
		assert_pig_latin("hello world", "ellohay orldway");
		assert_pig_latin("hello-hi", "ellohay-ihay");
		assert_pig_latin("Yes (no)", "Esyay (onay)");
		assert_pig_latin("Hello, ADORABLE world!", "Ellohay, ADORABLEWAY orldway!");
		assert_pig_latin("A T-shirt, I see.", "Away Tay-irtshay, Iway eesay.");
		assert_pig_latin("🦀 My name is मनीष. 📎", "🦀 Ymay amenay isway मनीष. 📎");
		assert_pig_latin("L'eau d'orange", "Eaul'ay oranged'ay");
		assert_pig_latin("P'sst ! Par ici !", "P'sstay ! Arpay iciway !");
		assert_pig_latin("Simon Example z״l", "Imonsay Exampleway z״lay");
		assert_pig_latin("Ploni Almoni a״h", "Oniplay Almoniway a״hway");
		assert_pig_latin("The Rebbe z״ya", "Ethay Ebberay az״yay");
	}

	#[test]
	fn custom_suffixes() {
		let transformer = PigLatinTransformer::new("yay", "-hay");
		let result = transformer.to_pig_latin("Hello, egg!");

		assert_eq!(result, "Ellohyay, egg-hay!");
	}
}
