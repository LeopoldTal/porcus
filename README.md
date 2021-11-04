# porcus

[Pig latin](https://en.wikipedia.org/wiki/Pig_Latin) for the whole Latin script.

## Motivations

- Enjoy Rust's excellent Unicode support
- Transform French text and IPA
- Be ridiculously extra about a toy project

## Use as an executable

````
porcus [-c consonant_suffix] [-v vowel_suffix]
````

Transforms standard input to pig latin on the standard output.

### Arguments

* `-c`, `--consonant` consonant_suffix: Suffix for words starting with a consonant [default: `ay`]
* `-v`, `--vowel` vowel_suffix: Suffix for words starting with a vowel [default: `way`]
* `-h`, `--help`: print help and exit
* `-V`, `--version`: print version and exit

## Use as a library

```rust
use porcus::PigLatinTransformer;

let transformer = PigLatinTransformer::default();
assert_eq!(transformer.to_pig_latin("Pig latin"), "Igpay atinlay");
```

All Latin script letters are supported.

```rust
# use porcus::PigLatinTransformer;
# let transformer = PigLatinTransformer::default();
assert_eq!(transformer.to_pig_latin("à l’œuf"), "àway œufl’ay");
assert_eq!(transformer.to_pig_latin("Česko"), "Eskočay");
```

You can also specify custom suffixes.

```rust
use porcus::PigLatinTransformer;

let transformer = PigLatinTransformer::new("eɪ", "weɪ");
assert_eq!(transformer.to_pig_latin("ə stɹɪŋ"), "əweɪ ɪŋstɹeɪ");
```

Build the full docs with `cargo doc` for more info.

## Develop

* Build: `cargo build`
* Run tests: `cargo test`
