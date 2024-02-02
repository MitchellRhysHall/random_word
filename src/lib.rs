//! # random_word
//!
//! The `random_word` crate provides an efficient way to generate
//! random words. Included words can be filtered by length or
//! first character.
//!
//! ## Usage
//! You **MUST** enable a crate language feature.
//! Crate language features are mandatory to reduce binary size.
//! Example for English in Cargo.toml:
//! ```toml
//! [dependencies]
//! random_word = { version = "0.4.2", features = ["en"] }
//! ```
//!
//! **Supported Languages:**
//! - English
//! - Spanish
//! - German
//! - French
//! - Japanese
//! - Chinese
//!

mod words;

use rand::{seq::SliceRandom, thread_rng};

/// ISO 639-1 language codes.
///
/// Each variant corresponds to a
/// set of words included in the binary.
///
/// You **MUST** enable the corresponding crate feature.
///
/// # Variants
///
/// * `De` - German. Requires enabling "de" feature.
/// * `En` - English. Requires enabling "en" feature.
/// * `Es` - Spanish. Requires enabling "es" feature.
/// * `Fr` - French. Requires enabling "fr" feature.
/// * `Ja` - Japanese. Requires enabling "ja" feature.
/// * `Zh` - Chinese. Requires enabling "zh" feature.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    #[cfg(feature = "de")]
    De,
    #[cfg(feature = "en")]
    En,
    #[cfg(feature = "es")]
    Es,
    #[cfg(feature = "fr")]
    Fr,
    #[cfg(feature = "ja")]
    Ja,
    #[cfg(feature = "zh")]
    Zh,
}

/// Returns all words with the given language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let words = random_word::all(Lang::En);
/// assert!(!words.is_empty());
/// ```
#[inline(always)]
pub fn all(lang: Lang) -> &'static [&'static str] {
    words::get(lang)
}

/// Generates a random word with the given language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let word = random_word::gen(Lang::En);
/// assert!(!word.is_empty());
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
/// use random_word::Lang;
/// let words = random_word::all_len(4, Lang::En);
/// assert!(words.is_some());
/// ```
#[inline(always)]
pub fn all_len(len: usize, lang: Lang) -> Option<&'static [&'static str]> {
    words::get_len(len, lang).map(|boxed| &**boxed)
}

/// Generates a random word with the given length and language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let word = random_word::gen_len(4, Lang::En);
/// assert!(word.is_some());
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
/// use random_word::Lang;
/// let words = random_word::all_starts_with('c', Lang::En);
/// assert!(words.is_some());
/// ```
#[inline(always)]
pub fn all_starts_with(char: char, lang: Lang) -> Option<&'static [&'static str]> {
    words::get_starts_with(char, lang).map(|boxed| &**boxed)
}

/// Generates a random word with the given starting character and language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let word = random_word::gen_starts_with('c', Lang::En);
/// assert!(word.is_some());
/// ```
#[inline(always)]
pub fn gen_starts_with(char: char, lang: Lang) -> Option<&'static str> {
    words::get_starts_with(char, lang)?
        .choose(&mut thread_rng())
        .copied()
}
