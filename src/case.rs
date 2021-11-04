//! Case detection and mapping.
//!
//! # Usage
//!
//! ```
//! use porcus::case::{self, Case};
//!
//! let s = "HELLO";
//! assert_eq!(case::detect_case(&s), Case::Upper);
//! assert_eq!(case::to_case(s, Case::Sentence), "Hello");
//! ```

use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

/// Case of a word.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Case {
	/// All characters are lowercase or uncased.
	Lower,
	/// All characters are uppercase or uncased.
	Upper,
	/// The first character is uppercase. All others are lowercase or uncased.
	Sentence,
	/// No consistent case pattern.
	Mixed,
}

impl Default for Case {
	fn default() -> Self {
		Self::Mixed
	}
}

impl fmt::Display for Case {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::Lower => "lowercase",
			Self::Upper => "UPPERCASE",
			Self::Sentence => "Sentencecase",
			Self::Mixed => "MixedCase",
		})
	}
}

/// Detects the case of a word.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use porcus::case::{Case, detect_case};
/// assert_eq!(detect_case("foobar"), Case::Lower);
/// assert_eq!(detect_case("FOOBAR"), Case::Upper);
/// assert_eq!(detect_case("Foobar"), Case::Sentence);
/// ```
///
/// Uncased characters do not affect the detection.
///
/// ```
/// # use porcus::case::{Case, detect_case};
/// assert_eq!(detect_case("foo_bar"), Case::Lower);
/// assert_eq!(detect_case("FOO_BAR"), Case::Upper);
/// assert_eq!(detect_case("Foo_bar"), Case::Sentence);
/// ```
///
/// Single-letter uppercase characters are considered sentence-case rather than uppercase.
///
/// ```
/// # use porcus::case::{Case, detect_case};
/// assert_eq!(detect_case("I"), Case::Sentence);
/// assert_eq!(detect_case("Å"), Case::Sentence);
/// ```
///
/// An all-uncased string is considered lowercase.
///
/// ```
/// # use porcus::case::{Case, detect_case};
/// assert_eq!(detect_case(""), Case::Lower);
/// assert_eq!(detect_case("42"), Case::Lower);
/// ```
///
/// When no specific case pattern is detected, the string is considered mixed-case.
///
/// ```
/// # use porcus::case::{Case, detect_case};
/// assert_eq!(detect_case("iPhone"), Case::Mixed);
/// assert_eq!(detect_case("SpOnGeBoB"), Case::Mixed);
/// ```
#[must_use]
pub fn detect_case(s: &str) -> Case {
	s.chars().next().map_or(Case::Lower, |first_char| {
		let first_is_lower = !first_char.is_uppercase();
		let first_is_upper = !first_char.is_lowercase();
		let rest_is_upper = s.chars().skip(1).all(|c| !c.is_lowercase());
		let rest_is_lower = s.chars().skip(1).all(|c| !c.is_uppercase());

		match (first_is_lower, first_is_upper, rest_is_lower, rest_is_upper) {
			(true, _, true, _) => Case::Lower,
			(_, true, true, _) => Case::Sentence,
			(_, true, _, true) => Case::Upper,
			_ => Case::Mixed,
		}
	})
}

/// Returns the equivalent of a string as the specified case.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use porcus::case::{Case, to_case};
/// assert_eq!(to_case("fooBAR", Case::Lower), "foobar");
/// assert_eq!(to_case("fooBAR", Case::Upper), "FOOBAR");
/// assert_eq!(to_case("fooBAR", Case::Sentence), "Foobar");
/// ```
///
/// Conversion to mixed case leaves the string unchanged.
///
/// ```
/// # use porcus::case::{Case, to_case};
/// assert_eq!(to_case("fooBAR", Case::Mixed), "fooBAR");
/// assert_eq!(to_case("foobar", Case::Mixed), "foobar");
/// ```
pub fn to_case<S: Into<String>>(s: S, case: Case) -> String {
	let s = s.into();
	match case {
		Case::Lower => s.to_lowercase(),
		Case::Upper => s.to_uppercase(),
		Case::Sentence => to_sentence_case(&s),
		Case::Mixed => s,
	}
}

fn to_sentence_case(s: &str) -> String {
	let mut graphemes = s.graphemes(true);
	let first = graphemes.next();

	if let Some(first_grapheme) = first {
		let rest = graphemes.as_str();
		format!("{}{}", first_grapheme.to_uppercase(), rest.to_lowercase())
	} else {
		String::new()
	}
}

#[cfg(test)]
mod test_detect_case {
	use super::*;

	#[test]
	fn lower() {
		assert_eq!(detect_case(""), Case::Lower);
		assert_eq!(detect_case("test"), Case::Lower);
		assert_eq!(detect_case("çà"), Case::Lower);
		assert_eq!(detect_case("测试"), Case::Lower);
		assert_eq!(detect_case("测试test"), Case::Lower);
		assert_eq!(detect_case("test测试"), Case::Lower);
	}

	#[test]
	fn upper() {
		assert_eq!(detect_case("TEST"), Case::Upper);
		assert_eq!(detect_case("ÇÀ"), Case::Upper);
		assert_eq!(detect_case("TEST测试"), Case::Upper);
		assert_eq!(detect_case("测试TEST"), Case::Upper);
	}

	#[test]
	fn sentence() {
		assert_eq!(detect_case("Test"), Case::Sentence);
		assert_eq!(detect_case("Çà"), Case::Sentence);
		assert_eq!(detect_case("X测试test"), Case::Sentence);
		assert_eq!(detect_case("I"), Case::Sentence);
	}

	#[test]
	fn mixed() {
		assert_eq!(detect_case("TESt"), Case::Mixed);
		assert_eq!(detect_case("tEST"), Case::Mixed);
		assert_eq!(detect_case("çÀ"), Case::Mixed);
		assert_eq!(detect_case("x测试Test"), Case::Mixed);
	}
}

#[cfg(test)]
mod test_to_case {
	use super::*;

	fn assert_case_transform(input: &str, case: Case, expected: &str) {
		let result = to_case(String::from(input), case);
		assert_eq!(result, expected);
	}

	#[test]
	fn lower() {
		assert_case_transform("", Case::Lower, "");
		assert_case_transform("tEsT", Case::Lower, "test");
		assert_case_transform("À", Case::Lower, "à");
		assert_case_transform("测试", Case::Lower, "测试");
	}

	#[test]
	fn upper() {
		assert_case_transform("", Case::Upper, "");
		assert_case_transform("tEsT", Case::Upper, "TEST");
		assert_case_transform("à", Case::Upper, "À");
		assert_case_transform("测试", Case::Upper, "测试");
	}

	#[test]
	fn sentence() {
		assert_case_transform("", Case::Sentence, "");
		assert_case_transform("tEsT", Case::Sentence, "Test");
		assert_case_transform("âgé", Case::Sentence, "Âgé");
		assert_case_transform("测试", Case::Sentence, "测试");
	}

	#[test]
	fn mixed() {
		assert_case_transform("", Case::Mixed, "");
		assert_case_transform("tEsT", Case::Mixed, "tEsT");
		assert_case_transform("âgé", Case::Mixed, "âgé");
		assert_case_transform("测试", Case::Mixed, "测试");
	}
}
