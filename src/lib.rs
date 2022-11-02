#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # random_word
//!
//! The `random_word` crate provides an efficient way to generate random 
//! english words. 
//! 
//! All words are compiled with the library pre-sorted, 
//! optimized for fast, zero allocation lookup.
//! 
//! ## Generating a random word
//!
//! ```rust
//! # fn gen() -> Result<(),&'static str> {
//! let word = random_word::gen();
//! # Ok(())
//! # }
//! ```

mod words_a_z;
mod words_len_asc;
mod tests;

use words_a_z::WORDS_A_Z;
use words_len_asc::WORDS_LEN_ASC;

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
    select_random(&WORDS_A_Z)
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
/// - the input parameter is not an alphabetic character
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
/// - the input parameter is not an alphabetic character
/// 
#[inline]
pub fn gen_all_starts_with(character: char) -> Result<&'static [&'static str], Error> {
    if !character.is_alphabetic() { 
        Err(Error::NonAlphabeticCharacter(character)) 
    }
    else {
        Ok(&WORDS_A_Z[match character {
            'a' | 'A' =>      0..10363,
            'b' | 'B' =>  10363..20327,
            'c' | 'C' =>  20327..36906,
            'd' | 'D' =>  36906..47297,
            'e' | 'E' =>  47297..54481,
            'f' | 'F' =>  54481..61608,
            'g' | 'G' =>  61608..67478,
            'h' | 'H' =>  67478..73971,
            'i' | 'I' =>  73971..80599,
            'j' | 'J' =>  80599..82077,
            'k' | 'K' =>  82077..83932,
            'l' | 'L' =>  83932..89246,
            'm' | 'M' =>  89246..99191,
            'n' | 'N' =>  99191..103643,
            'o' | 'O' => 103643..109700,
            'p' | 'P' => 109700..124756,
            'q' | 'Q' => 124756..125605,
            'r' | 'R' => 125605..136123,
            's' | 'S' => 136123..155842,
            't' | 'T' => 155842..164837,
            'u' | 'U' => 164837..170068,
            'v' | 'V' => 170068..172926,
            'w' | 'W' => 172926..176844,
            'x' | 'X' => 176844..176996,
            'z' | 'Z' => 177586..178187,
            'y' | 'Y' => 176996..177586,
                _     => unreachable!()
        }])
    }
}

/// Returns an alphabetically ordered slice of all words 
/// with the specified length.
/// 
/// # Example
///
/// ```rust
/// # fn gen_all_len() -> Result<(),&'static str> {
/// let word_list = random_word::gen_all_len(5);
/// # Ok(())
/// # }
/// ```
/// # Errors
/// 
/// This function fails if:
/// - the length parameter is less than 2 or greater than 15
/// 
#[inline]
pub fn gen_all_len(length: usize) -> Option<&'static [&'static str]> {
    if length < 2 || length > 15 { 
        None 
    }
    else {
        Some(&WORDS_LEN_ASC[match length {
            2  =>      0..99,
            3  =>    100..1102,
            4  =>   1102..5097,
            5  =>   5097..13982,
            6  =>  13982..29702,
            7  =>  29702..53652,
            8  =>  53652..83336,
            9  =>  83336..112417,
            10 => 112417..134698,
            11 => 134698..150832,
            12 => 150832..162231,
            13 => 162231..169974,
            14 => 169974..175030,
            15 => 175030..178187,
            _  => unreachable!()
        }])
    }
}

/// Returns a reference to a word with the specified length.
///
/// # Example
///
/// ```rust
/// # fn gen_len() -> Result<(),&'static str> {
/// let word = random_word::gen_len(5);
/// # Ok(())
/// # }
/// ```
/// 
/// # Errors
/// 
/// This function fails if:
/// - the length parameter is less than 2 or greater than 15
/// 
#[inline]
pub fn gen_len(length: usize) -> Option<&'static str> {
    Some(select_random(&gen_all_len(length)?))
}

/// Returns an alphabetically ordered array of 178,187 english words.
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
    &WORDS_A_Z
}