use phf::phf_set;

/// Latin vowels, excluding modifiers
pub const VOWELS: phf::Set<char> = phf_set! {
	// Basic Latin
	'A', 'E', 'I', 'O', 'U', // Uppercase
	'a', 'e', 'i', 'o', 'u', // Lowercase
	// Latin 1 supplement
	'ª', 'º', // Symbol
	'Æ', 'Ø', 'æ', 'ø', 'ı', 'Ĳ', 'ĳ', 'Œ', 'œ', // Letter
	// Extended B
	'Ǝ', 'Ə', 'Ɛ', 'Ɩ', 'Ɨ', 'Ɵ', 'Ʊ', // Non-european & historic
	'ǝ', // Phonetic & historic
	'Ⱥ', // Sencoten
	'Ȣ', 'ȣ', 'Ʉ', 'Ɇ', 'ɇ', // Misc
	// IPA
	'ɐ', 'ɑ', 'ɒ', // a-like
	'ɘ', 'ə', 'ɚ', 'ɛ', 'ɜ', 'ɝ', 'ɞ', // e-like
	'ɨ', 'ɩ', 'ɪ', // i-like
	'ɵ', 'ɶ', 'ɷ', // o-like
	'ʉ', 'ʊ', // u-like
	// Phonetic letters
	'ᴀ', 'ᴁ', 'ᴂ', // a-like
	'ᴇ', 'ᴈ',  // e-like
	'ᴉ', // i-like
	'ᴏ', 'ᴐ', 'ᴑ', 'ᴒ', 'ᴓ', 'ᴔ', 'ᴕ', 'ᴖ', 'ᴗ', // o-like
	'ᴜ', 'ᴝ', 'ᴞ', 'ᵫ', // u-like
	'ᵻ', 'ᵼ', 'ᵾ', 'ᵿ', // Phonetic sign
	'ᶏ', 'ᶐ', 'ᶒ', 'ᶓ', 'ᶔ', 'ᶕ', 'ᶖ', 'ᶗ', 'ᶙ', // Phonetic retroflex hook
	'ẚ', // General extension
	'ⁱ', // Superscript
	'ₐ', 'ₑ', 'ₒ', 'ₔ', // Subscript
	// Extended C
	'ⱥ', // Orthographic addition
	'Ɑ', 'Ɐ', 'Ɒ', // Misc
	'ⱸ', 'ⱺ', 'ⱻ', // UPA
	// Extended D
	// Medievalist
	'Ꜳ', 'ꜳ', 'Ꜵ', 'ꜵ', 'Ꜷ', 'ꜷ', 'Ꜹ', 'ꜹ', 'Ꜻ', 'ꜻ', 'Ꜽ', 'ꜽ', // a-like
	'Ꝋ', 'ꝋ', 'Ꝍ', 'ꝍ', 'Ꝏ', 'ꝏ', // o-like
	'Ꝫ', 'ꝫ', 'Ꝭ', 'ꝭ', 'ꝸ', // Abbreviations
	'Ꞛ', 'ꞛ', 'Ꞝ', 'ꞝ', 'Ꞟ', 'ꞟ', // Volapük
	'Ɜ', // Letters
	'Ɪ', // West African
	'Ꞷ', 'ꞷ', // African
	'Ꞹ', 'ꞹ', // Mazahua
	'Ꞻ', 'ꞻ', 'Ꞽ', 'ꞽ', 'Ꞿ', 'ꞿ', // Ugaritic & Egyptologic
	'ꟷ', // Celtic
	'ꟹ', // IPA
	'ꟾ', // Roman
	// Extended E
	// German dialects
	'ꬰ', 'ꬱ', // a-like
	'ꬲ', 'ꬳ', 'ꬴ', // e-like
	'ꬽ', 'ꬾ', 'ꬿ', 'ꭀ', 'ꭁ', 'ꭂ', 'ꭃ', 'ꭄ', // o-like
	'ꭎ', 'ꭏ', 'ꭐ', 'ꭑ', 'ꭒ', // u-like
	'ꭠ', 'ꭡ', 'ꭢ', 'ꭣ', // Sakha
	'ꭤ', // American
	// Fullwidth
	'Ａ', 'Ｅ', 'Ｉ', 'Ｏ', 'Ｕ', // Uppercase
	'ａ', 'ｅ', 'ｉ', 'ｏ', 'ｕ', // Lowercase
};

pub const AMBIGUOUS_VOWELS: phf::Set<char> = phf_set! {
	'Y', 'y', 'Ƴ', 'ƴ', 'Ɏ', 'ɏ', 'ʎ', 'ʏ', 'Ỿ', 'ỿ', 'Ｙ', 'ｙ', 'ꭚ',
};

pub const CONSONANT_LIKE_PUNCTUATION: phf::Set<char> = phf_set! {
	'\'', '’', '＇', '·', '՟', '״', '‧'
};
