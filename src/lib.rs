//! # random_word
//!
//! The `random_word` crate provides an efficient way to generate random words.
//! Included words can be filtered by length or first character.
//!
//! You MUST enable a crate language feature.
//! Example in cargo.toml:
//! random_word = { version = "0.4.0", features = ["en"] }
//!
//! **Supported Languages:**
//! - English
//! - Spanish
//! - German
//! - French
//!

pub mod lang;
mod words;

use lang::Lang;
use rand::{seq::SliceRandom, thread_rng};

/// Returns all words with the given language.
///
/// # Example
/// ```
/// use random_word::lang::Lang;
#[cfg_attr(
    feature = "de",
    doc = "let words = random_word::all(Lang::De);\nassert!(!words.is_empty());"
)]
#[cfg_attr(
    feature = "en",
    doc = "let words = random_word::all(Lang::En);\nassert!(!words.is_empty());"
)]
#[cfg_attr(
    feature = "es",
    doc = "let words = random_word::all(Lang::En);\nassert!(!words.is_empty());"
)]
#[cfg_attr(
    feature = "fr",
    doc = "let words = random_word::all(Lang::Fr);\nassert!(!words.is_empty());"
)]
/// ```
#[inline(always)]
pub fn all(lang: Lang) -> &'static Box<[&'static str]> {
    words::get(lang)
}

/// Generates a random word with the given language.
///
/// # Example
/// ```
/// use random_word::lang::Lang;
#[cfg_attr(
    feature = "de",
    doc = "let word = random_word::gen(Lang::De);\nassert!(!word.is_empty());"
)]
#[cfg_attr(
    feature = "en",
    doc = "let word = random_word::gen(Lang::En);\nassert!(!word.is_empty());"
)]
#[cfg_attr(
    feature = "es",
    doc = "let word = random_word::gen(Lang::En);\nassert!(!word.is_empty());"
)]
#[cfg_attr(
    feature = "fr",
    doc = "let word = random_word::gen(Lang::Fr);\nassert!(!word.is_empty());"
)]
/// ```
#[inline(always)]
pub fn gen(lang: Lang) -> &'static str {
    words::get(lang)
        .choose(&mut thread_rng())
        .expect("array is empty")
}

/// Returns all words with the given length and language.
///
/// # Example
/// ```
/// use random_word::lang::Lang;
#[cfg_attr(
    feature = "de",
    doc = "let words = random_word::all_len(4, Lang::De);\nassert!(words.is_some());"
)]
#[cfg_attr(
    feature = "en",
    doc = "let words = random_word::all_len(4, Lang::En);\nassert!(words.is_some());"
)]
#[cfg_attr(
    feature = "es",
    doc = "let words = random_word::all_len(4, Lang::En);\nassert!(words.is_some());"
)]
#[cfg_attr(
    feature = "fr",
    doc = "let words = random_word::all_len(4, Lang::Fr);\nassert!(words.is_some());"
)]
/// ```
#[inline(always)]
pub fn all_len(len: usize, lang: Lang) -> Option<&'static Box<[&'static str]>> {
    words::get_len(len, lang)
}

/// Generates a random word with the given length and language.
///
/// # Example
/// ```
/// use random_word::lang::Lang;
#[cfg_attr(
    feature = "de",
    doc = "let word = random_word::gen_len(4, Lang::De);\nassert!(word.is_some());"
)]
#[cfg_attr(
    feature = "en",
    doc = "let word = random_word::gen_len(4, Lang::En);\nassert!(word.is_some());"
)]
#[cfg_attr(
    feature = "es",
    doc = "let word = random_word::gen_len(4, Lang::En);\nassert!(word.is_some());"
)]
#[cfg_attr(
    feature = "fr",
    doc = "let word = random_word::gen_len(4, Lang::Fr);\nassert!(word.is_some());"
)]
/// ```
#[inline(always)]
pub fn gen_len(len: usize, lang: Lang) -> Option<&'static str> {
    words::get_len(len, lang)?
        .choose(&mut thread_rng())
        .copied()
}

/// Returns all words with the given starting character and language.
///
/// # Example
/// ```
/// use random_word::lang::Lang;
#[cfg_attr(
    feature = "de",
    doc = "let words = random_word::all_starts_with('c', Lang::De);\nassert!(words.is_some());"
)]
#[cfg_attr(
    feature = "en",
    doc = "let words = random_word::all_starts_with('c', Lang::En);\nassert!(words.is_some());"
)]
#[cfg_attr(
    feature = "es",
    doc = "let words = random_word::all_starts_with('c', Lang::En);\nassert!(words.is_some());"
)]
#[cfg_attr(
    feature = "fr",
    doc = "let words = random_word::all_starts_with('c', Lang::Fr);\nassert!(words.is_some());"
)]
/// ```
#[inline(always)]
pub fn all_starts_with(char: char, lang: Lang) -> Option<&'static Box<[&'static str]>> {
    words::get_starts_with(char, lang)
}

/// Generates a random word with the given starting character and language.
///
/// # Example
/// ```
/// use random_word::lang::Lang;
#[cfg_attr(
    feature = "de",
    doc = "let word = random_word::gen_starts_with('c', Lang::De);\nassert!(word.is_some());"
)]
#[cfg_attr(
    feature = "en",
    doc = "let word = random_word::gen_starts_with('c', Lang::En);\nassert!(word.is_some());"
)]
#[cfg_attr(
    feature = "es",
    doc = "let word = random_word::gen_starts_with('c', Lang::En);\nassert!(word.is_some());"
)]
#[cfg_attr(
    feature = "fr",
    doc = "let word = random_word::gen_starts_with('c', Lang::Fr);\nassert!(word.is_some());"
)]
/// ```
#[inline(always)]
pub fn gen_starts_with(char: char, lang: Lang) -> Option<&'static str> {
    words::get_starts_with(char, lang)?
        .choose(&mut thread_rng())
        .copied()
}
