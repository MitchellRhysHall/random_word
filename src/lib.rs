//! # random_word
//!
//! The `random_word` crate provides an efficient way to generate random words.
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

use std::{sync::Arc, fmt::Display};
use miniz_oxide::inflate::decompress_to_vec;
use rand::{seq::SliceRandom, thread_rng};
use ahash::AHashMap;

#[derive(Debug)]
pub enum Error {
    ZeroLengthWord,
    WordLengthExceededMax,
    DecompressionFailed,
    InvalidUtf8,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Words {
    all: Vec<Arc<String>>,
    len_map: Vec<Vec<Arc<String>>>,
    startswith_map: AHashMap<char, Vec<Arc<String>>>,
    len_startswith_map: AHashMap<(usize, char), Vec<Arc<String>>>,
}

impl Words {
    const MAX_WORD_LEN: usize = 50;

    /// Creates a new instance of `Words` with the provided newline delimited source string slice.
    ///
    /// The function parses the provided string and creates a set of words which
    /// is then stored and used for lookups.
    ///
    /// # Arguments
    ///
    /// * `source` - A newline delimited string slice that holds the source words to be stored.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the source string was parsed successfully, 
    /// otherwise it returns an `Err` variant indicating the error.
    pub fn new(source: &str) -> Result<Self> {
        let mut all = Vec::new();
        let mut startswith_map = AHashMap::new();
        let mut len_startswith_map = AHashMap::new();

        let mut len_map = vec![Vec::new(); Self::MAX_WORD_LEN];

        let mut current_max_len = 0;

        for line in source.lines() {
            let len = line.chars().count();

            let start_char = line.chars()
                .next()
                .ok_or(Error::ZeroLengthWord)?;

            let line_ptr = Arc::new(line.to_string());

            len_map.get_mut(len)
                .ok_or(Error::WordLengthExceededMax)?
                .push(line_ptr.clone());

            startswith_map.entry(start_char)
                .or_insert_with(Vec::new)
                .push(line_ptr.clone());

            len_startswith_map.entry((len, start_char))
                .or_insert_with(Vec::new)
                .push(line_ptr.clone());
            
            all.push(line_ptr);

            if len > current_max_len { current_max_len = len }
        }

        len_map.truncate(current_max_len);

        Ok(Self { all, len_map, startswith_map, len_startswith_map })
    }

    /// Returns all the words stored in the `Words` instance.
    ///
    /// # Returns
    ///
    /// A reference to a `Vec<Arc<String>>` holding all stored words.
    #[inline(always)]
    pub fn all(&self) -> &Vec<Arc<String>> {
        &self.all
    }

    /// Returns all the words of a specific length stored in the `Words` instance.
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the words to be returned.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to a `Vec<Arc<String>>` holding 
    /// all stored words of the specified length. If no such words are stored, 
    /// it returns `None`.
    #[inline(always)]
    pub fn all_len(&self, len: usize) -> Option<&Vec<Arc<String>>> {
        self.len_map.get(len)
    }

    /// Returns all the words starting with a specific character stored 
    /// in the `Words` instance.
    ///
    /// # Arguments
    ///
    /// * `char` - The initial character of the words to be returned.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to a `Vec<Arc<String>>` holding 
    /// all stored words starting with the specified character. 
    /// If no such words are stored, it returns `None`.
    #[inline(always)]
    pub fn all_starts_with(&self, char: char) -> Option<&Vec<Arc<String>>> {
        self.startswith_map.get(&char)
    }

    /// Returns all the words of a specific length and starting with a 
    /// specific character stored in the `Words` instance.
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the words to be returned.
    ///
    /// * `char` - The initial character of the words to be returned.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to a `Vec<Arc<String>>` holding 
    /// all stored words of the specified length and starting with 
    /// the specified character. If no such words are stored, it returns `None`.
    #[inline(always)]
    pub fn all_len_starts_with(&self, len: usize, char: char) -> Option<&Vec<Arc<String>>> {
        self.len_startswith_map.get(&(len, char))
    }

    /// Generates and returns a random word from the stored words.
    ///
    /// This function uses a `thread_rng` to randomly select a word
    /// from the stored words in the `Words` structure. It returns an
    /// `Option<&Arc<String>>`. If there are words stored in the structure, 
    /// it returns `Some(&Arc<String>)` where `&Arc<String>` is a reference to the 
    /// selected word. If no words are stored, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use random_word::{Words, Lang};
    ///
    /// let words = Words::from(&Lang::En);
    /// let random_word = words.gen();
    ///
    /// match random_word {
    ///     Some(word) => println!("Generated word: {}", word),
    ///     None => println!("No words stored."),
    /// }
    /// ```
    #[inline(always)]
    pub fn gen(&self) -> Option<&Arc<String>> {
        self.all().choose(&mut thread_rng())
    }

    /// Generates and returns a random word of a specified length 
    /// from the stored words.
    ///
    /// This function uses a `thread_rng` to randomly select a word of a
    /// specified length from the stored words in the `Words` structure.
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the word to be generated.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the generated word.
    /// If no words of the specified length are stored, it returns `None`.
    #[inline(always)]
    pub fn gen_len(&self, len: usize) -> Option<&Arc<String>> {
        self.all_len(len)?.choose(&mut thread_rng())
    }

    /// Generates and returns a random word starting with a specified 
    /// character from the stored words.
    ///
    /// This function uses a `thread_rng` to randomly select a word starting
    /// with a specified character from the stored words in the `Words` structure.
    ///
    /// # Arguments
    ///
    /// * `char` - The initial character of the word to be generated.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the generated word.
    /// If no words starting with the specified character are stored, it returns `None`.
    #[inline(always)]
    pub fn gen_starts_with(&self, char: char) -> Option<&Arc<String>> {
        self.all_starts_with(char)?.choose(&mut thread_rng())
    }

    /// Generates and returns a random word of a specified length and 
    /// starting with a specified character from the stored words.
    ///
    /// This function uses a `thread_rng` to randomly select a word of a 
    /// specified length and starting with a specified character from 
    /// the stored words in the `Words` structure.
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the word to be generated.
    ///
    /// * `char` - The initial character of the word to be generated.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the generated word.
    /// If no words of the specified length and starting with the 
    /// specified character are stored, it returns `None`.
    #[inline(always)]
    pub fn gen_len_starts_with(&self, len: usize, char: char) -> Option<&Arc<String>> {
        self.all_len_starts_with(len, char)?.choose(&mut thread_rng())
    }

    pub fn from(lang_code: &Lang) -> Result<Self> {
        let gz: &[u8] = match lang_code {
            #[cfg(feature = "de")]
            Lang::De =>  include_bytes!("gz/de.gz"),
            #[cfg(feature = "en")]
            Lang::En =>  include_bytes!("gz/en.gz"),
            #[cfg(feature = "es")]
            Lang::Es =>  include_bytes!("gz/es.gz"),
            #[cfg(feature = "fr")]
            Lang::Fr =>  include_bytes!("gz/fr.gz")
        };
    
        let word_bytes = decompress_to_vec(gz).map_err(|_| Error::DecompressionFailed)?;
        let decompressed = String::from_utf8(word_bytes).map_err(|_| Error::InvalidUtf8)?;
    
        Self::new(&decompressed)
    }
}

/// ISO 639-1 language codes.
///
/// This enum can be used to select the language of the word data that you want to load using the `Words::from` function.
///
/// Each variant of this enum corresponds to a different language. The language files are included in the binary,
/// and each variant of `Lang` corresponds to one of these files. 
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
    Fr
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Lang::De => "German",
            Lang::En => "English",
            Lang::Es => "Spanish",
            Lang::Fr => "French",
        })
    }
}