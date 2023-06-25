use random_word::{Lang, Words};
use unicase::UniCase;

static LANGS: [Lang; 4] = [Lang::De, Lang::En, Lang::Es, Lang::Fr];
static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

#[test]
fn test_gen() {
    for lang in &LANGS {
        let words = Words::from(*lang);
        let word = words.gen().unwrap();
        assert!(word.len() > 0, "Generated word should not be empty.");
    }
}

#[test]
fn test_gen_starts_with() {
    for lang in &LANGS {
        let words = Words::from(*lang);
        for ch in &VOWELS {
            let word = words.gen_starts_with(*ch);
            assert!(
                word.is_some(),
                "Should return Some for a valid language and character."
            );
            let word_str = word.unwrap();
            assert!(word_str.len() > 0, "Generated word should not be empty.");
            assert_eq!(
                word_str.chars().next().unwrap(),
                *ch,
                "Generated word should start with the specified character."
            );
        }
    }
}

#[test]
fn test_all_starts_with() {
    let supported_char = 'a';
    let unsupported_char = '1';

    for lang in &LANGS {
        let words = Words::from(*lang);

        let word_list = words.all_starts_with(unsupported_char);
        assert!(
            word_list.is_none(),
            "Should return None for an unsupported character."
        );

        let word_list = words.all_starts_with(supported_char);
        assert!(
            word_list.is_some(),
            "Should return Some for a supported character."
        );

        let word_list = word_list.unwrap();

        // Check that the array is sorted
        let mut sorted_word_list = word_list.to_vec();
        sorted_word_list.sort_by_key(|s| UniCase::new(s.clone().to_string()));
        assert_eq!(
            word_list,
            &sorted_word_list[..],
            "Returned array should be sorted."
        );

        // Check that all words start with the specified character
        for word in word_list {
            assert_eq!(
                word.chars().next().unwrap(),
                supported_char,
                "All words should start with the specified character."
            );
        }
    }
}

#[test]
fn test_all_len() {
    for lang in &LANGS {
        for len in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] {
            let words = Words::from(*lang);

            if let Some(word_list) = words.all_len(len) {
                // Check that the array is sorted
                let mut sorted_word_list = word_list.to_vec();
                sorted_word_list.sort_by_key(|s| UniCase::new(s.clone().to_string()));
                assert_eq!(
                    word_list,
                    &sorted_word_list[..],
                    "Returned array should be sorted."
                );

                // Check that all words are length
                for word in word_list {
                    assert_eq!(
                        word.chars().count(),
                        len,
                        "All words should be of equal length"
                    );
                }
            }
        }
    }
}

#[test]
fn test_all() {
    for lang in &LANGS {
        let words = Words::from(*lang);

        let word_list = words.all();

        // Check if the result is not empty
        assert!(!word_list.is_empty());

        // Check that the array is sorted
        let mut sorted_word_list = word_list.to_vec();
        sorted_word_list.sort_by_key(|s| UniCase::new(s.clone().to_string()));
        assert_eq!(
            word_list,
            &sorted_word_list[..],
            "Returned array should be sorted."
        );
    }
}
