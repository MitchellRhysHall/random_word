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
use words::WORDS;

fn select_random(array: &'static [&'static str]) -> &'static str {
    array[rand::random::<usize>() % array.len()]
}

#[test]
fn random_test() {
    let array = &["1","2","3","4","5"];
    // All options from array should be pushed into this set.
    // This ensures that the random selection function
    // is capable of selecting all available items from array.
    let mut set = std::collections::HashSet::new();
    (0..500000).for_each(|_| {
        let choice = select_random(array);
        set.insert(choice);
        assert!(!choice.is_empty(), "Random choice should not be empty");
        assert!(array.contains(&choice), "Random choice should be from input array");
    });
    assert_eq!(set.len(), array.len(), "Set length should match array length");
    array.iter().for_each(|item| {
        assert!(set.contains(item), "Set should contain array item");
    });
}

/// Returns a reference to a randomly generated english word.
///
/// # Examples
///
/// ```rust
/// # fn gen() -> Result<(),&'static str> {
/// let word = random_word::gen();
/// # Ok(())
/// # }
/// ```
///
pub fn gen() -> &'static str {
    select_random(&WORDS)
}

#[test]
fn gen_test() {
    let mut word_list = vec![];
    (0..50).for_each(|_| {
        let word = gen();
        assert!(!word_list.contains(&word), 
            "List of up to 50 previously generated words contains duplicates.");
        assert!(!word.is_empty(), "generated word should not be empty");
        word_list.push(word);
    })
}

/// Returns a reference to a randomly generated english word, 
/// filtering by first character.
///
/// # Examples
///
/// ```rust
/// # fn gen_starts_with() -> Result<(),&'static str> {
/// let word = random_word::gen_starts_with('s');
/// # Ok(())
/// # }
/// ```
///
pub fn gen_starts_with(character: char) -> &'static str {
    select_random(&gen_all_starts_with(character))
}

#[test]
fn gen_starts_with_test() {
    ('a'..='z').for_each(|character| {
        (0..5000).for_each(|_|{
            let word = gen_starts_with(character);
            assert!(!word.is_empty(), "word should not be empty");
            assert!(word.starts_with(character), "word should start with {character}");
        })
    })
}

/// Returns an alphabetically ordered slice to all words 
/// that begin with the specified character.
///
/// # Examples
///
/// ```rust
/// # fn gen_all_starts_with() -> Result<(),&'static str> {
/// let t_words_slice = random_word::gen_all_starts_with('t');
/// # Ok(())
/// # }
/// ```
///
pub fn gen_all_starts_with(character: char) -> &'static [&'static str] {
    &WORDS[match character {
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
        'y' | 'Y' => 176996..177586,
        'z' | 'Z' => 177586..178187,
         _  => panic!("character is not alphabetic!")
    }]
}

#[test]
fn gen_all_starts_with_test() {
    ('a'..='z').chain('A'..'Z').for_each(|character| {
        let word_list = gen_all_starts_with(character);
        word_list.iter().for_each(|word| {
            let character = character.to_ascii_lowercase();
            assert!(!word.is_empty(), "word should not be empty");
            assert!(word.starts_with(character), "word should start with {character}");
        })
    })
}

/// Returns a reference to 178,187 english words in alphabetical order.
///
/// # Examples
///
/// ```rust
/// # fn gen_all() -> Result<(),[&'static str; 178187]> {
/// let word_list = random_word::gen_all();
/// # Ok(())
/// # }
/// ```
///
pub fn gen_all() -> &'static [&'static str; 178187] {
    &WORDS
}

#[test]
fn gen_all_test() {
    let word_list = gen_all();
    word_list.iter().for_each(|word| {
        assert!(!word.is_empty(), 
            "word should not be empty in word list");
        assert!(word.chars().all(|c| ('a'..='z').contains(&c)),
            "word should only contain lowercase alphabetical characters");
    })
}

#[test]
fn gen_all_indexes_from_first_char() {
    let word_list = gen_all();
    ('a'..='z').for_each(|c| {
        let i = word_list.iter().position(|w| w.starts_with(c))
            .expect("words starting with any alphabetic character 
                should be represented in words array");
        //println!("first index starting with {c} is {i}");
    });
}