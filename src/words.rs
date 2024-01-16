use ahash::AHashMap;
use brotli::Decompressor;
use once_cell::sync::Lazy;
use std::io::{Cursor, Read};

use crate::Lang;

pub(crate) type Words = Box<[&'static str]>;

macro_rules! generate_lazy_db_from_file {
    ($file_stem:ident) => {
        paste::paste! {
            static [<$file_stem:upper _COMPRESSED>]: Lazy<String> = Lazy::new(|| {
                let compressed_bytes = include_bytes!(concat!("br/", stringify!($file_stem), ".br"));
                let cursor = Cursor::new(compressed_bytes);
                let mut decompressor = Decompressor::new(cursor, 4096);

                let mut decompressed_bytes = Vec::new();
                decompressor.read_to_end(&mut decompressed_bytes).expect("Decompression failed");

                let decompressed_string = String::from_utf8(decompressed_bytes)
                    .expect("Decompression resulted in invalid UTF-8");

                decompressed_string
            });

            static [<$file_stem:upper>]: Lazy<Words> = Lazy::new(|| {
                [<$file_stem:upper _COMPRESSED>].lines().collect()
            });

            static [<$file_stem:upper _LEN>]: Lazy<AHashMap<usize, Words>> = Lazy::new(|| {
                let mut map = AHashMap::new();

                for &word in [<$file_stem:upper>].iter() {
                    let len = word.chars().count();
                    map.entry(len).or_insert_with(Vec::new).push(word);
                }

                map.into_iter().map(|(k, v)| (k, v.into_boxed_slice())).collect()
            });

            static [<$file_stem:upper _STARTS_WITH>]: Lazy<AHashMap<char, Words>> = Lazy::new(|| {
                let mut map = AHashMap::new();

                for &word in [<$file_stem:upper>].iter() {
                    let first = word.chars().nth(0).expect("empty word");
                    map.entry(first).or_insert_with(Vec::new).push(word);
                }

                map.into_iter().map(|(k, v)| (k, v.into_boxed_slice())).collect()
            });
        }
    };
}

#[cfg(feature = "de")]
generate_lazy_db_from_file!(de);
#[cfg(feature = "en")]
generate_lazy_db_from_file!(en);
#[cfg(feature = "es")]
generate_lazy_db_from_file!(es);
#[cfg(feature = "fr")]
generate_lazy_db_from_file!(fr);
#[cfg(feature = "ja")]
generate_lazy_db_from_file!(ja);
#[cfg(feature = "zh")]
generate_lazy_db_from_file!(zh);

#[inline(always)]
pub(crate) fn get(lang: Lang) -> &'static Words {
    match lang {
        #[cfg(feature = "de")]
        Lang::De => &DE,
        #[cfg(feature = "en")]
        Lang::En => &EN,
        #[cfg(feature = "es")]
        Lang::Es => &ES,
        #[cfg(feature = "fr")]
        Lang::Fr => &FR,
        #[cfg(feature = "ja")]
        Lang::Ja => &JA,
        #[cfg(feature = "zh")]
        Lang::Zh => &ZH,
    }
}

#[inline(always)]
pub(crate) fn get_len(len: usize, lang: Lang) -> Option<&'static Words> {
    match lang {
        #[cfg(feature = "de")]
        Lang::De => DE_LEN.get(&len),
        #[cfg(feature = "en")]
        Lang::En => EN_LEN.get(&len),
        #[cfg(feature = "es")]
        Lang::Es => ES_LEN.get(&len),
        #[cfg(feature = "fr")]
        Lang::Fr => FR_LEN.get(&len),
        #[cfg(feature = "ja")]
        Lang::Ja => JA_LEN.get(&len),
        #[cfg(feature = "zh")]
        Lang::Zh => ZH_LEN.get(&len),
    }
}

#[inline(always)]
pub(crate) fn get_starts_with(char: char, lang: Lang) -> Option<&'static Words> {
    match lang {
        #[cfg(feature = "de")]
        Lang::De => DE_STARTS_WITH.get(&char),
        #[cfg(feature = "en")]
        Lang::En => EN_STARTS_WITH.get(&char),
        #[cfg(feature = "es")]
        Lang::Es => ES_STARTS_WITH.get(&char),
        #[cfg(feature = "fr")]
        Lang::Fr => FR_STARTS_WITH.get(&char),
        #[cfg(feature = "ja")]
        Lang::Ja => JA_STARTS_WITH.get(&char),
        #[cfg(feature = "zh")]
        Lang::Zh => ZH_STARTS_WITH.get(&char),
    }
}
