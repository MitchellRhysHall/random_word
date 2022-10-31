#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # random_word
//!
//! The `random_word` crate provides a simple way to generate random 
//! english words.
//! 
//! All 178,187 words are compiled with the library.
//! 
//! ## Generating a random word
//!
//! ```rust
//! use random_word;
//! # fn generate() -> Result<(),&'static str> {
//! let word = generate();
//! # Ok(())
//! # }
//! ```

use rand::random;
mod words;
use words::WORDS;

/// Returns a reference to a randomly generated english word.
///
/// # Examples
///
/// ```rust
/// use random_word;
/// # fn generate() -> Result<(),&'static str> {
/// let word = generate();
/// # Ok(())
/// # }
/// ```
///
pub fn generate() -> &'static str {
    WORDS[random::<usize>() % WORDS.len()]
}

/// Test function for 'generate' method
#[test]
fn generate_test() {
    let mut word_list = vec![];
    (0..50).for_each(|_| {
        let word = generate();
        assert!(!word_list.contains(&word), 
            "List of up to 50 previously generated words contains duplicates.");
        assert!(!word.is_empty(), "generated word should not be empty");
        word_list.push(word);
    })
}

/// Returns a reference to 178,187 english words.
///
/// # Examples
///
/// ```rust
/// use random_word;
/// # fn generate_all() -> Result<(),[&'static str; 178187]> {
/// let word_list = generate_all();
/// # Ok(())
/// # }
/// ```
///
pub fn generate_all() -> &'static [&'static str; 178187] {
    &WORDS
}

/// Test function for 'generate all' method
#[test]
fn generate_all_test() {
    let word_list = generate_all();
    // let word_hash_set = word_list.iter()
    //     .collect::<std::collections::HashSet<_>>();
    // assert_eq!(word_list.len(), word_hash_set.len(), 
    //     "There should be no duplicates words");
    word_list.iter().for_each(|word| {
        assert!(!word.is_empty(), 
            "word should not be empty in word list");
        assert!(word.chars().all(|c| ('a'..='z').contains(&c)),
            "word should only contain lowercase alphabetical characters");
    })
}