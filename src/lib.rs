//! # random_word
//!
//! The `random_word` crate provides an efficient way to generate random
//! english words.
//!
//! You MUST enable a language crate feature.
//! Example in cargo.toml:
//! random_word = { version = "0.4.0", features = ["en"] }
//! 
//! You can optionally enable features "all" to use all 
//! languages, however the binary size will be very large.
//! 
//! ## Generating a random word
//!
//! ```rust
//! use random_word::*;
//! let word = gen(&Lang::En);
//! ```
//! ## Generating a random word starting with 'c'
//!
//! ```rust
//! use random_word::*;
//! let word = gen_starts_with(&'c', &Lang::En);
//! assert!(word.is_some());
//! ```
//! ## Get all available words
//!
//! ```rust
//! use random_word::*;
//! let word_list = all(&Lang::En);
//! ```

mod tests;

use std::hash::Hash;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use rand::Rng;

#[inline]
fn select_random<'a>(array: &'a [&'a str]) -> &'a str {
    array[rand::thread_rng().gen_range(0..array.len())]
}

///
/// Builds the code which builds static hashmaps from compressed data
/// 
macro_rules! lang_map {
    ($(($lang:ident, $str:expr)),* $(,)?) => {
        $(
            #[cfg(feature = $str)]
            pub(crate) static $lang: Lazy<String> = Lazy::new(|| {
                String::from_utf8(
                    miniz_oxide::inflate::decompress_to_vec((
                        include_bytes!(
                            concat!("gz/", $str, ".gz"))))
                            .unwrap())
                            .unwrap()
            });
        )*

        pub(crate) static ALL_MAP: Lazy<HashMap<Lang, Vec<&str>>> = Lazy::new(|| {
            let mut map = HashMap::new();

            $(
                #[cfg(feature = $str)]
                let _ = map.insert(Lang::$lang, $lang.lines().collect());
            )*

            map
        });

        fn init_map<'a, F, K>(key_fn: F) -> HashMap<Lang, HashMap<K, Vec<&'a str>>>
        where
            F: Fn(&str) -> K,
            K: Eq + Hash,
        {
            let mut map = HashMap::new();

            $(
                #[cfg(feature = $str)]
                let _ = map.insert(
                    Lang::$lang, 
                    $lang.lines()
                        .fold(HashMap::new(), |mut map, word| {
                            map.entry(key_fn(word)).or_insert_with(Vec::new).push(word);
                            map
                        }));
            )*

            map
        }

        pub(crate) static STARTSWITH_MAP: Lazy<HashMap<Lang, HashMap<char, Vec<&str>>>> = Lazy::new(|| 
            init_map(|word: &str| word.chars().next().unwrap()));

        pub(crate) static LEN_MAP: Lazy<HashMap<Lang, HashMap<usize, Vec<&str>>>> = Lazy::new(|| 
            init_map(|word: &str| word.len()));

        pub(crate) static LEN_STARTSWITH_MAP: Lazy<HashMap<Lang, HashMap<(usize, char), Vec<&str>>>> = Lazy::new(|| 
            init_map(|word: &str| (word.len(), word.chars().next().unwrap())));
    };
}

lang_map!(
    (De, "de"),
    (En, "en"),
    (Es, "es"),
    (Fr, "fr"),
);

/// All feature enabled languages. 
/// You MUST enable a crate language feature.
/// 
/// Example in cargo.toml:
/// random_word = { version = "0.4.0", features = ["en"] }
/// 
/// You can enable features "all" to use all 
/// languages, however the binary size will be very large.
/// 
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Lang {
    #[cfg(feature = "de")]
    De,
    #[cfg(feature = "en")]
    En,
    #[cfg(feature = "es")]
    Es,
    #[cfg(feature = "fr")]
    Fr,
}

/// Returns a reference to a randomly generated word.
///
/// # Example
///
/// ```rust
/// use random_word::*;
/// let word = gen(&Lang::En);
/// ```
/// 
/// # Errors
/// 
/// Returns none if no words are found matching the input criteria.
///
#[inline]
pub fn gen<'a>(lang: &Lang) -> &'a str {
    select_random(all(lang))
}

/// Returns a reference to a word that begins with the
/// specified character.
/// 
/// **WARNING CASE-SENSITIVE:**
/// German nouns are upper-case
///
/// # Example
///
/// ```rust
/// use random_word::*;
/// 
/// let word = gen_starts_with(&'s', &Lang::En);
/// assert!(word.is_some());
/// 
/// let word = gen_starts_with(&'Ü', &Lang::De);
/// assert!(word.is_some());
/// 
/// let word = gen_starts_with(&'é', &Lang::Fr);
/// assert!(word.is_some());
/// 
/// let word = gen_starts_with(&'É', &Lang::Fr);
/// assert!(word.is_none());
/// 
/// ```
///
/// # Errors
///
/// Returns none if no words are found matching the input character.
///
#[inline]
pub fn gen_starts_with<'a>(char: &char, lang: &Lang) -> Option<&'a str> {
    Some(select_random(&all_starts_with(char, lang)?))
}

/// Returns an alphabetically ordered slice of all words
/// that begin with the specified character.
///
/// **WARNING CASE-SENSITIVE:**
/// German nouns are upper-case
/// 
/// # Example
///
/// ```rust
/// use random_word::*;
/// let word_list = all_starts_with(&'t', &Lang::En);
/// assert!(word_list.is_some());
/// ```
///
/// # Errors
///
/// Returns none if no words are found matching the input character.
///
#[inline]
pub fn all_starts_with<'a>(char: &char, lang: &Lang) -> Option<&'a [&'a str]> {
    STARTSWITH_MAP
        .get(lang)
        .and_then(|m| m
            .get(char))
        .map(|v| v
            .as_slice())
}

/// Returns an alphabetically ordered slice of all words
/// with the specified length.
///
/// # Example
///
/// ```rust
/// use random_word::*;
/// let word_list = all_len(&5, &Lang::En);
/// assert!(word_list.is_some());
/// ```
/// # Errors
///
/// Returns none if no words are found matching the input length.
///
#[inline]
pub fn all_len<'a>(len: &usize, lang: &Lang) -> Option<&'a [&'a str]> {
    LEN_MAP
        .get(lang)
        .and_then(|m| m
            .get(len))
        .map(|v| v
            .as_slice())
}

/// Returns an alphabetically ordered slice of all words
/// with the specified length, and starting with
/// the specified character
///
/// **WARNING CASE-SENSITIVE:**
/// German nouns are upper-case
/// 
/// # Example
///
/// ```rust
/// use random_word::*;
/// let word_list = all_len_starts_with(&5, &'a', &Lang::En);
/// assert!(word_list.is_some());
/// ```
/// # Errors
///
/// Returns none if no words are found matching the input length and character.
///
#[inline]
pub fn all_len_starts_with<'a>(len: &usize, char: &char, lang: &Lang) -> Option<&'a [&'a str]> {
    LEN_STARTSWITH_MAP
        .get(lang)
        .and_then(|m| m
            .get(&(*len, *char)))
        .map(|v| v
            .as_slice())
}

/// Returns a reference to a word with the specified length,
/// and starting with the specified character.
///
/// # Example
///
/// ```rust
/// use random_word::*;
/// let word = gen_len_starts_with(&9, &'p', &Lang::En);
/// assert!(word.is_some());
/// ```
/// # Errors
///
/// Returns none if no words are found matching the input length and character.
///
#[inline]
pub fn gen_len_starts_with<'a>(len: &usize, char: &char, lang: &Lang) -> Option<&'a str> {
    Some(select_random(&all_len_starts_with(len, char, lang)?))
}

/// Returns a reference to a word with the specified length.
///
/// # Example
///
/// ```rust
/// use random_word::*;
/// let word = gen_len(&5, &Lang::En);
/// assert!(word.is_some());
/// ```
///
/// # Errors
///
/// Returns none if no words are found matching the input length.
///
#[inline]
pub fn gen_len<'a>(len: &usize, lang: &Lang) -> Option<&'a str> {
    Some(select_random(&all_len(len, lang)?))
}

/// Returns an alphabetically ordered slice of words.
///
/// # Example
///
/// ```rust
/// use random_word::*;
/// let word_list = all(&Lang::En);
/// ```
///
#[inline]
pub fn all<'a>(lang: &Lang) -> &'a [&'a str] {
    ALL_MAP.get(lang)
        .map(|v| v
            .as_slice())
            .expect("Language variants should only be exposed when the crate feature is enabled")
}
