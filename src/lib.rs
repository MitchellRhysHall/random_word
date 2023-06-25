//! # random_word
//!
//! The `random_word` crate provides an efficient way to generate random words.
//! The words can be filtered by length and/or the first character.
//!
//! **Supported Languages:**
//! - English
//! - Spanish
//! - German
//! - French
//!
//! You MUST enable a crate language feature.
//! Example in cargo.toml:
//! random_word = { version = "0.4.0", features = ["en"] }
//!

use ahash::AHashMap;
use miniz_oxide::inflate::decompress_to_vec;
use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    WordLengthExceededMax,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Words<'a> {
    all: Box<[&'a str]>,
    len_map: Box<[Box<[&'a str]>]>,
    startswith_map: AHashMap<char, Box<[&'a str]>>,
    len_startswith_map: AHashMap<(usize, char), Box<[&'a str]>>,
}

impl<'a> Words<'a> {
    const MAX_WORD_LEN: usize = 50;

    pub fn new(source: &'a str, delim: char) -> Result<Self> {
        let mut all = Vec::new();
        let mut startswith_map = AHashMap::new();
        let mut len_startswith_map = AHashMap::new();

        let mut len_map = vec![Vec::new(); Self::MAX_WORD_LEN];

        let mut current_max_len = 0;

        for line in source.split(delim) {
            let start_char = if let Some(start_char) = line.chars().next() {
                start_char
            } else {
                continue;
            };

            let len = line.chars().count();

            len_map
                .get_mut(len)
                .ok_or(Error::WordLengthExceededMax)?
                .push(line);

            startswith_map
                .entry(start_char)
                .or_insert_with(Vec::new)
                .push(line);

            len_startswith_map
                .entry((len, start_char))
                .or_insert_with(Vec::new)
                .push(line);

            all.push(line);

            if len > current_max_len {
                current_max_len = len
            }
        }

        len_map.truncate(current_max_len);

        let all = all.into_boxed_slice();
        let len_map = len_map
            .into_iter()
            .map(|inner_vec| inner_vec.into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let startswith_map = startswith_map
            .into_iter()
            .map(|(k, inner_vec)| (k, inner_vec.into_boxed_slice()))
            .collect();

        let len_startswith_map = len_startswith_map
            .into_iter()
            .map(|(k, inner_vec)| (k, inner_vec.into_boxed_slice()))
            .collect();

        Ok(Self {
            all,
            len_map,
            startswith_map,
            len_startswith_map,
        })
    }

    #[inline(always)]
    pub fn all(&'a self) -> &'a [&'a str] {
        &self.all
    }

    #[inline(always)]
    pub fn all_len(&'a self, len: usize) -> Option<&'a [&'a str]> {
        self.len_map.get(len).map(|b| &**b)
    }

    #[inline(always)]
    pub fn all_starts_with(&'a self, char: char) -> Option<&'a [&'a str]> {
        self.startswith_map.get(&char).map(|b| &**b)
    }

    #[inline(always)]
    pub fn all_len_starts_with(&'a self, len: usize, char: char) -> Option<&'a [&'a str]> {
        self.len_startswith_map.get(&(len, char)).map(|b| &**b)
    }

    #[inline(always)]
    pub fn gen(&self) -> Option<&'a str> {
        self.all.choose(&mut thread_rng()).copied()
    }

    #[inline(always)]
    pub fn gen_len(&self, len: usize) -> Option<&'a str> {
        self.len_map.get(len)?.choose(&mut thread_rng()).copied()
    }

    #[inline(always)]
    pub fn gen_starts_with(&self, char: char) -> Option<&'a str> {
        self.startswith_map
            .get(&char)?
            .choose(&mut thread_rng())
            .copied()
    }

    #[inline(always)]
    pub fn gen_len_starts_with(&self, len: usize, char: char) -> Option<&'a str> {
        self.len_startswith_map
            .get(&(len, char))?
            .choose(&mut thread_rng())
            .copied()
    }
}

impl<'a> From<Lang> for Words<'a> {
    fn from(lang_code: Lang) -> Self {
        static WORD_MAP: Lazy<AHashMap<Lang, Lazy<String>>> = Lazy::new(|| {
            fn decompress(bytes: &[u8]) -> String {
                let word_bytes = decompress_to_vec(bytes).unwrap();
                String::from_utf8(word_bytes).unwrap()
            }

            let mut map: AHashMap<Lang, Lazy<String>> = AHashMap::new();

            #[cfg(feature = "de")]
            map.insert(
                Lang::De,
                Lazy::new(|| decompress(include_bytes!("gz/de.gz"))),
            );

            #[cfg(feature = "en")]
            map.insert(
                Lang::En,
                Lazy::new(|| decompress(include_bytes!("gz/en.gz"))),
            );

            #[cfg(feature = "es")]
            map.insert(
                Lang::Es,
                Lazy::new(|| decompress(include_bytes!("gz/es.gz"))),
            );

            #[cfg(feature = "fr")]
            map.insert(
                Lang::Fr,
                Lazy::new(|| decompress(include_bytes!("gz/fr.gz"))),
            );

            map
        });

        Self::new(&WORD_MAP.get(&lang_code).unwrap(), '\n').unwrap()
    }
}

/// ISO 639-1 language codes.
///
/// This enum is used to select the language of the word data that you want to load using the `Words::from` function.
///
/// Each variant of this enum corresponds to a different language of words included in the binary.
///
/// Note that each variant requires enabling the corresponding crate feature.
///
/// # Examples
///
/// ```rust
/// use random_word::Lang;
///
/// let language = Lang::En;  // English language.
/// ```
///
/// # Variants
///
/// * `De` - Represents the German language. Requires enabling the "de" feature.
/// * `En` - Represents the English language. Requires enabling the "en" feature.
/// * `Es` - Represents the Spanish language. Requires enabling the "es" feature.
/// * `Fr` - Represents the French language. Requires enabling the "fr" feature.
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
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Lang::De => "German",
                Lang::En => "English",
                Lang::Es => "Spanish",
                Lang::Fr => "French",
            }
        )
    }
}
