use once_cell::sync::Lazy;
use std::str;

/// A list of known english words
/// Does not include words "a" or "i".
static DATA: &[u8; 1758840] = include_bytes!("words.csv");
static LENGTH: usize = 178187;

static DATA_VEC: Lazy<Vec<&'static str>> = Lazy::new(|| {
    str::from_utf8(DATA)
        .expect("CSV data is not valid UTF-8")
        .lines()
        .map(|line| line.split(',').collect::<Vec<_>>()[0])
        .collect()
});

pub(crate) static WORDS_A_Z: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let mut data = DATA_VEC.clone();
    if data.len() != LENGTH {
        panic!("Unexpected word count")
    }
    data.sort();
    data
});

pub(crate) static WORDS_LEN_ASC: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let mut data = DATA_VEC.clone();
    if data.len() != LENGTH {
        panic!("Unexpected word count")
    }
    data.sort_by_key(|a| a.len());
    data
});
