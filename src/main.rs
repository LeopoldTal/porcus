use porcus;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
	let g = &"test".graphemes(true).collect::<Vec<&str>>();
	println!("{:?}", porcus::get_char_type_at(g, 2));
	println!("{:?}", porcus::latin::CONSONANT_LIKE_PUNCTUATION);
}
