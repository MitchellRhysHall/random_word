#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # random_word
//!
//! The `random_word` crate provides an efficient way to generate random 
//! english words. 
//! 
//! This crate is optimized for reduced memory usage.
//! Library size approx ~450kb.
//! 
//! All 178,187 words are compiled with the library.
//! All functions return static references.
//! 
//! ## Generating a random word
//!
//! ```rust
//! # fn gen() -> Result<(),&'static str> {
//! let word = random_word::gen();
//! # Ok(())
//! # }
//! ```

mod words;
mod tests;
use words::WORDS;

/// Represents all possible errors.
#[derive(Debug)]
pub enum Error {
    /// Occurs when an invalid char parameter is passed to a function.
    NonAlphabeticCharacter(char),
}

#[inline]
fn select_random(array: &'static [&'static str]) -> &'static str {
    array[rand::random::<usize>() % array.len()]
}

/// Returns a reference to a randomly generated english word.
///
/// # Example
///
/// ```rust
/// # fn gen() -> Result<(),&'static str> {
/// let word = random_word::gen();
/// # Ok(())
/// # }
/// ```
///
#[inline]
pub fn gen() -> &'static str {
    select_random(&WORDS)
}

/// Returns a reference to a word that begins with the 
/// specified character.
///
/// # Example
///
/// ```rust
/// # fn gen_starts_with() -> Result<(),&'static str> {
/// let word = random_word::gen_starts_with('s');
/// # Ok(())
/// # }
/// ```
/// 
/// # Errors
/// 
/// This function fails if:
/// - The input parameter is not an alphabetic character
/// 
#[inline]
pub fn gen_starts_with(character: char) -> Result<&'static str, Error> {
    Ok(select_random(&gen_all_starts_with(character)?))
}

/// Returns an alphabetically ordered slice of all words 
/// that begin with the specified character.
///
/// # Example
///
/// ```rust
/// # fn gen_all_starts_with() -> Result<(),&'static str> {
/// let word_list = random_word::gen_all_starts_with('t');
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// 
/// This function fails if:
/// - The input parameter is not an alphabetic character
/// 
#[inline]
pub fn gen_all_starts_with(character: char) -> Result<&'static [&'static str], Error> {
    match character {
        'a' | 'A' =>      Ok(&WORDS[0..10363]),
        'b' | 'B' =>  Ok(&WORDS[10363..20327]),
        'c' | 'C' =>  Ok(&WORDS[20327..36906]),
        'd' | 'D' =>  Ok(&WORDS[36906..47297]),
        'e' | 'E' =>  Ok(&WORDS[47297..54481]),
        'f' | 'F' =>  Ok(&WORDS[54481..61608]),
        'g' | 'G' =>  Ok(&WORDS[61608..67478]),
        'h' | 'H' =>  Ok(&WORDS[67478..73971]),
        'i' | 'I' =>  Ok(&WORDS[73971..80599]),
        'j' | 'J' =>  Ok(&WORDS[80599..82077]),
        'k' | 'K' =>  Ok(&WORDS[82077..83932]),
        'l' | 'L' =>  Ok(&WORDS[83932..89246]),
        'm' | 'M' =>  Ok(&WORDS[89246..99191]),
        'n' | 'N' =>  Ok(&WORDS[99191..103643]),
        'o' | 'O' => Ok(&WORDS[103643..109700]),
        'p' | 'P' => Ok(&WORDS[109700..124756]),
        'q' | 'Q' => Ok(&WORDS[124756..125605]),
        'r' | 'R' => Ok(&WORDS[125605..136123]),
        's' | 'S' => Ok(&WORDS[136123..155842]),
        't' | 'T' => Ok(&WORDS[155842..164837]),
        'u' | 'U' => Ok(&WORDS[164837..170068]),
        'v' | 'V' => Ok(&WORDS[170068..172926]),
        'w' | 'W' => Ok(&WORDS[172926..176844]),
        'x' | 'X' => Ok(&WORDS[176844..176996]),
        'y' | 'Y' => Ok(&WORDS[176996..177586]),
        'z' | 'Z' => Ok(&WORDS[177586..178187]),
         c  => Err(Error::NonAlphabeticCharacter(c))
    }
}

/// Returns a reference to 178,187 english words in alphabetical order.
///
/// # Example
///
/// ```rust
/// # fn gen_all() -> Result<(),[&'static str; 178187]> {
/// let word_list = random_word::gen_all();
/// # Ok(())
/// # }
/// ```
///
#[inline]
pub fn gen_all() -> &'static [&'static str; 178187] {
    &WORDS
}