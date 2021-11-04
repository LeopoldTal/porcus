//! Pig latin for the whole Latin script.
//!
//! # Usage
//!
//! ```
//! use porcus::PigLatinTransformer;
//!
//! let transformer = PigLatinTransformer::default();
//! assert_eq!(transformer.to_pig_latin("Pig latin"), "Igpay atinlay");
//! ```
//!
//! All Latin script letters are supported.
//!
//! ```
//! # use porcus::PigLatinTransformer;
//! # let transformer = PigLatinTransformer::default();
//! assert_eq!(transformer.to_pig_latin("à l’œuf"), "àway œufl’ay");
//! assert_eq!(transformer.to_pig_latin("Česko"), "Eskočay");
//! ```
//!
//! You can also specify custom suffixes.
//!
//! ```
//! use porcus::PigLatinTransformer;
//!
//! let transformer = PigLatinTransformer::new("eɪ", "weɪ");
//! assert_eq!(transformer.to_pig_latin("ə stɹɪŋ"), "əweɪ ɪŋstɹeɪ");
//! ```

pub mod case;
pub mod char_type;
pub mod latin;

mod pig_latin;
pub use crate::pig_latin::PigLatinTransformer;
