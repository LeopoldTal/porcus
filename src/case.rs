use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
pub enum Case {
	Lower,
	Upper,
	Title,
	Mixed,
}

pub fn detect_case(s: &str) -> Case {
	if let Some(first_char) = s.chars().next() {
		let rest_is_upper = s.chars().skip(1).all(|c| !c.is_lowercase());
		let rest_is_lower = s.chars().skip(1).all(|c| !c.is_uppercase());

		match (first_char.is_uppercase(), rest_is_upper, rest_is_lower) {
			(true, true, _) => Case::Upper,
			(true, _, true) => Case::Title,
			(false, _, true) => Case::Lower,
			_ => Case::Mixed,
		}
	} else {
		Case::Mixed
	}
}

pub fn to_case(s: String, case: Case) -> String {
	match case {
		Case::Lower => s.to_lowercase(),
		Case::Upper => s.to_uppercase(),
		Case::Title => to_title_case(s),
		Case::Mixed => s,
	}
}

fn to_title_case(s: String) -> String {
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
	}

	#[test]
	fn title() {
		assert_eq!(detect_case("Test"), Case::Title);
		assert_eq!(detect_case("Çà"), Case::Title);
		assert_eq!(detect_case("X测试test"), Case::Title);
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
	fn title() {
		assert_case_transform("", Case::Title, "");
		assert_case_transform("tEsT", Case::Title, "Test");
		assert_case_transform("âgé", Case::Title, "Âgé");
		assert_case_transform("测试", Case::Title, "测试");
	}

	#[test]
	fn mixed() {
		assert_case_transform("", Case::Mixed, "");
		assert_case_transform("tEsT", Case::Mixed, "tEsT");
		assert_case_transform("âgé", Case::Mixed, "âgé");
		assert_case_transform("测试", Case::Mixed, "测试");
	}
}
