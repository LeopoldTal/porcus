use crate::latin::{AMBIGUOUS_VOWELS, CONSONANT_LIKE_PUNCTUATION, VOWELS};
use unicode_normalization::UnicodeNormalization;
use unicode_script::UnicodeScript;

#[derive(Debug, PartialEq)]
pub enum CharType {
	Vowel,
	Consonant,
	Ambiguous,
	NonLatin,
	None,
}

pub fn get_char_type_at(graphemes: &Vec<&str>, index: usize) -> CharType {
	if let Some(first_char) = get_first_nfd_char_of_grapheme_at(graphemes, index) {
		if VOWELS.contains(&first_char) {
			return CharType::Vowel;
		}
		if AMBIGUOUS_VOWELS.contains(&first_char) {
			return CharType::Ambiguous;
		}
		if CONSONANT_LIKE_PUNCTUATION.contains(&first_char) {
			return CharType::Consonant;
		}
		let script = first_char.script().full_name();
		if script == "Latin" {
			CharType::Consonant
		} else {
			CharType::NonLatin
		}
	} else {
		CharType::None
	}
}

fn get_first_nfd_char_of_grapheme_at(graphemes: &Vec<&str>, index: usize) -> Option<char> {
	if let Some(grapheme) = graphemes.get(index) {
		grapheme.nfd().next()
	} else {
		None
	}
}

#[cfg(test)]
mod test_get_char_type_at {
	use super::*;

	#[test]
	fn empty() {
		assert_eq!(get_char_type_at(&vec![], 0), CharType::None);
		assert_eq!(get_char_type_at(&vec![""], 0), CharType::None);
		assert_eq!(get_char_type_at(&vec!["a"], 42), CharType::None);
	}

	#[test]
	fn vowels() {
		let graphemes = &vec![
			"a", "e", "i", "o", "u", "A", "å", "ã", "é", "Î", "ö", "ø", "œ", "ə",
		];
		for grapheme_index in 0..graphemes.len() {
			assert_eq!(get_char_type_at(graphemes, grapheme_index), CharType::Vowel);
		}
	}

	#[test]
	fn consonants() {
		let graphemes = &vec!["b", "B", "ç", "Đ", "þ", "ñ", "ß", "ʔ", "Ⅰ"];
		for grapheme_index in 0..graphemes.len() {
			assert_eq!(
				get_char_type_at(graphemes, grapheme_index),
				CharType::Consonant
			);
		}
	}

	#[test]
	fn ambiguous() {
		let graphemes = &vec!["y", "Y", "Ÿ", "ȳ", "ỿ", "Ｙ"];
		for grapheme_index in 0..graphemes.len() {
			assert_eq!(
				get_char_type_at(graphemes, grapheme_index),
				CharType::Ambiguous
			);
		}
	}

	#[test]
	fn non_latin() {
		let graphemes = &vec![" ", "\"", ",", ".", "π"];
		for grapheme_index in 0..graphemes.len() {
			assert_eq!(
				get_char_type_at(graphemes, grapheme_index),
				CharType::NonLatin
			);
		}
	}

	#[test]
	fn treat_special_punctuation_as_consonants() {
		let graphemes = &vec!["'", "’", "·", "״"];
		for grapheme_index in 0..graphemes.len() {
			assert_eq!(
				get_char_type_at(graphemes, grapheme_index),
				CharType::Consonant
			);
		}
	}

	#[test]
	fn treat_modifiers_as_consonants() {
		let graphemes = &vec!["ʰ", "ᵃ", "ʸ"];
		for grapheme_index in 0..graphemes.len() {
			assert_eq!(
				get_char_type_at(graphemes, grapheme_index),
				CharType::Consonant
			);
		}
	}
}
