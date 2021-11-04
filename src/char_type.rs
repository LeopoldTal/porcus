//! Classifies graphemes as vowels or consonants.
//!
//! # Usage
//!
//! ```
//! use porcus::char_type::{CharType, get_char_type_at};
//!
//! assert_eq!(get_char_type_at(&vec!["ʃ"], 0), CharType::Consonant);
//! ```
//!
//! # Classification details
//!
//! `Y` and its variants are classified as [ambiguous](CharType::Ambiguous).
//!
//! Characters outside the Latin script are classified as [non-latin](CharType::NonLatin), with
//! the exception of [a few punctuation marks](CONSONANT_LIKE_PUNCTUATION) which are considered
//! to be consonants.
//!
//! The empty string also receives [its own special classification](CharType::Empty).

use crate::latin::{AMBIGUOUS_VOWELS, CONSONANT_LIKE_PUNCTUATION, VOWELS};
use std::fmt;
use unicode_normalization::UnicodeNormalization;
use unicode_script::UnicodeScript;

/// Vowel-or-consonant classification of a grapheme.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum CharType {
	/// Latin vowel, e.g. `A`, `æ`, `ő`, `ɛ`.
	Vowel,
	/// Latin consonant, e.g. `B`, `ç`, `ł`, `ʁ`.
	///
	/// Also includes [some punctuation](CONSONANT_LIKE_PUNCTUATION) which may appear inside
	/// words, e.g. `'`.
	Consonant,
	/// Latin letter which may be a vowel or a consonant, e.g. `Y`.
	Ambiguous,
	/// Non-Latin script, e.g. ` `, `.`, `1`, `的`.
	NonLatin,
	/// Empty string.
	Empty,
}

impl fmt::Display for CharType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::Vowel => "vowel",
			Self::Consonant => "consonant",
			Self::Ambiguous => "ambiguous",
			Self::NonLatin => "non-latin",
			Self::Empty => "empty",
		})
	}
}

/// Classifies the grapheme at the specified index as a vowel or a consonant.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use porcus::char_type::{CharType, get_char_type_at};
/// let v = &vec!["B", "a", "y", "."];
/// assert_eq!(get_char_type_at(v, 0), CharType::Consonant);
/// assert_eq!(get_char_type_at(v, 1), CharType::Vowel);
/// assert_eq!(get_char_type_at(v, 2), CharType::Ambiguous);
/// assert_eq!(get_char_type_at(v, 3), CharType::NonLatin);
/// ```
///
/// [Some punctuation](CONSONANT_LIKE_PUNCTUATION) which can occur inside of words is also
/// treated as a consonant.
///
/// ```
/// # use porcus::char_type::{CharType, get_char_type_at};
/// let v = &vec!["'", "״"];
/// assert_eq!(get_char_type_at(v, 0), CharType::Consonant);
/// assert_eq!(get_char_type_at(v, 1), CharType::Consonant);
/// ```
///
/// NFC and NFD forms are treated identically.
///
/// ```
/// # use porcus::char_type::{CharType, get_char_type_at};
/// let v = &vec!["ç", "c\u{0327}"];
/// assert_eq!(get_char_type_at(v, 0), CharType::Consonant);
/// assert_eq!(get_char_type_at(v, 1), CharType::Consonant);
/// ```
///
/// This function expects its first argument to contain single grapheme clusters as returned by
/// [`UnicodeSegmentation::graphemes`](https://unicode-rs.github.io/unicode-segmentation/unicode_segmentation/trait.UnicodeSegmentation.html#tymethod.graphemes),
/// but will classify the first grapheme of any string, and return a
/// [special value](CharType::Empty) for empty strings.
///
/// ```
/// # use porcus::char_type::{CharType, get_char_type_at};
/// let v = &vec!["", "abc"];
/// assert_eq!(get_char_type_at(v, 0), CharType::Empty);
/// assert_eq!(get_char_type_at(v, 1), CharType::Vowel);
/// ```
///
/// # Bugs
///
/// Only the Latin script is handled.
///
/// ```
/// # use porcus::char_type::{CharType, get_char_type_at};
/// let v = &vec!["α", "ב"];
/// assert_eq!(get_char_type_at(v, 0), CharType::NonLatin);
/// assert_eq!(get_char_type_at(v, 1), CharType::NonLatin);
/// ```
///
/// Which characters are consonants and which are vowels depends on the orthography of the
/// language. Classification choices made here are largely relative to English orthography, and
/// wrong for other languages.
///
/// ```
/// # use porcus::char_type::{CharType, get_char_type_at};
/// let v = &vec!["w"]; // a Welsh vowel
/// assert_eq!(get_char_type_at(v, 0), CharType::Consonant);
/// ```
#[must_use]
pub fn get_char_type_at(graphemes: &[&str], index: usize) -> CharType {
	get_first_nfd_char_of_grapheme_at(graphemes, index).map_or(CharType::Empty, |first_char| {
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
	})
}

fn get_first_nfd_char_of_grapheme_at(graphemes: &[&str], index: usize) -> Option<char> {
	graphemes
		.get(index)
		.and_then(|grapheme| grapheme.nfd().next())
}

#[cfg(test)]
mod test_get_char_type_at {
	use super::*;

	#[test]
	fn empty() {
		assert_eq!(get_char_type_at(&vec![], 0), CharType::Empty);
		assert_eq!(get_char_type_at(&vec![""], 0), CharType::Empty);
		assert_eq!(get_char_type_at(&vec!["a"], 42), CharType::Empty);
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
