mod classify_char;
pub use crate::classify_char::{get_char_type_at, CharType};

pub mod case;
pub mod latin;

mod pig_latin;
pub use crate::pig_latin::{get_default_transformer, PigLatinTransformer};
