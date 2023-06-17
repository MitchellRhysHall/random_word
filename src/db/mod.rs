use crate::Lang;
use std::collections::HashMap;

use miniz_oxide::inflate::decompress_to_vec;
use once_cell::sync::Lazy;

fn decompress_to_string(bytes: &[u8]) -> String {
    String::from_utf8(decompress_to_vec(bytes).unwrap()).unwrap()
}

fn into_len_map<'a>(data: &'a str) -> HashMap<usize, Vec<&'a str>> {
    let mut map = HashMap::new();
    for line in data.lines() {
        let word = line.split(',').next().unwrap();
        let key = word.len();
        map.entry(key).or_insert_with(Vec::new).push(word);
    }
    map
}

fn into_len_starts_with_map<'a>(data: &'a str) -> HashMap<(usize, char), Vec<&'a str>> {
    let mut map = HashMap::new();
    for line in data.lines() {
        let word = line.split(',').next().unwrap();
        let key = (word.len(), word.chars().next().unwrap());
        map.entry(key).or_insert_with(Vec::new).push(word);
    }
    map
}

fn into_starts_with_map<'a>(data: &'a str) -> HashMap<char, Vec<&'a str>> {
    let mut map = HashMap::new();
    for line in data.lines() {
        let word = line.split(',').next().unwrap();
        let key = word.chars().next().unwrap();
        map.entry(key).or_insert_with(Vec::new).push(word);
    }
    map
}

fn into_vec<'a>(data: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();
    for line in data.lines() {
        let word = line.split(',').next().unwrap();
        vec.push(word);
    }
    vec
}

pub(crate) static ALL_MAP: Lazy<HashMap<Lang, Vec<&str>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    #[cfg(feature = "de")]
    let _ = map.insert(Lang::De, into_vec(&GERMAN));

    #[cfg(feature = "en")]
    let _ = map.insert(Lang::En, into_vec(&ENGLISH));

    #[cfg(feature = "es")]
    let _ = map.insert(Lang::Es, into_vec(&SPANISH));
    
    #[cfg(feature = "fr")]
    let _ = map.insert(Lang::Fr, into_vec(&FRENCH));
    
    map
});

pub(crate) static STARTSWITH_MAP: Lazy<HashMap<Lang, HashMap<char, Vec<&str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    #[cfg(feature = "de")]
    let _ = map.insert(Lang::De, into_starts_with_map(&GERMAN));

    #[cfg(feature = "en")]
    let _ = map.insert(Lang::En, into_starts_with_map(&ENGLISH));

    #[cfg(feature = "es")]
    let _ = map.insert(Lang::Es, into_starts_with_map(&SPANISH));
    
    #[cfg(feature = "fr")]
    let _ = map.insert(Lang::Fr, into_starts_with_map(&FRENCH));
    
    map
});

pub(crate) static LEN_MAP: Lazy<HashMap<Lang, HashMap<usize, Vec<&str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    #[cfg(feature = "de")]
    let _ = map.insert(Lang::De, into_len_map(&GERMAN));

    #[cfg(feature = "en")]
    let _ = map.insert(Lang::En, into_len_map(&ENGLISH));

    #[cfg(feature = "es")]
    let _ = map.insert(Lang::Es, into_len_map(&SPANISH));
    
    #[cfg(feature = "fr")]
    let _ = map.insert(Lang::Fr, into_len_map(&FRENCH));
    
    map
});


pub(crate) static LEN_STARTSWITH_MAP: Lazy<HashMap<Lang, HashMap<(usize, char), Vec<&str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    #[cfg(feature = "de")]
    let _ = map.insert(Lang::De, into_len_starts_with_map(&GERMAN));

    #[cfg(feature = "en")]
    let _ = map.insert(Lang::En, into_len_starts_with_map(&ENGLISH));

    #[cfg(feature = "es")]
    let _ = map.insert(Lang::Es, into_len_starts_with_map(&SPANISH));
    
    #[cfg(feature = "fr")]
    let _ = map.insert(Lang::Fr, into_len_starts_with_map(&FRENCH));
    
    map
});

#[cfg(feature = "de")]
pub(crate) static GERMAN: Lazy<String> = Lazy::new(|| {
    decompress_to_string(include_bytes!("gz/de.gz"))
});

#[cfg(feature = "en")]
pub(crate) static ENGLISH: Lazy<String> = Lazy::new(|| {
    decompress_to_string(include_bytes!("gz/en.gz"))
});

#[cfg(feature = "es")]
pub(crate) static SPANISH: Lazy<String> = Lazy::new(|| {
    decompress_to_string(include_bytes!("gz/es.gz"))
});

#[cfg(feature = "fr")]
pub(crate) static FRENCH: Lazy<String> = Lazy::new(|| {
    decompress_to_string(include_bytes!("gz/fr.gz"))
});
