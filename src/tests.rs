#[cfg(feature = "all")]
#[cfg(test)]
mod Tests {
    use super::*;
    use crate::*;
    use unicase::UniCase;

    static LANGS: [Lang; 4] = [Lang::De, Lang::En, Lang::Es, Lang::Fr];
    static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    #[test]
    fn test_gen() {
        for lang in &LANGS {
            let word = gen(lang);
            assert!(word.len() > 0, "Generated word should not be empty.");
        }
    }

    #[test]
    fn test_gen_starts_with() {
        for lang in &LANGS {
            for ch in &VOWELS {
                let word = gen_starts_with(ch, lang);
                assert!(word.is_some(), "Should return Some for a valid language and character.");
                let word_str = word.unwrap();
                assert!(word_str.len() > 0, "Generated word should not be empty.");
                assert_eq!(word_str.chars().next().unwrap(), *ch, "Generated word should start with the specified character.");
            }
        }
    }

    #[test]
    fn test_all_starts_with() {
        let supported_char = 'a';
        let unsupported_char = '1';

        for lang in &LANGS {
            let word_list = all_starts_with(&unsupported_char, lang);
            assert!(word_list.is_none(), "Should return None for an unsupported character.");

            let word_list = all_starts_with(&supported_char, lang);
            assert!(word_list.is_some(), "Should return Some for a supported character.");

            let word_list = word_list.unwrap();

            // Check that the array is sorted
            let mut sorted_word_list = word_list.to_vec();
            sorted_word_list.sort_by_key(|s| UniCase::new(s.clone()));
            assert_eq!(word_list, &sorted_word_list[..], "Returned array should be sorted.");

            // Check that all words start with the specified character
            for word in word_list {
                assert_eq!(word.chars().next().unwrap(), supported_char, "All words should start with the specified character.");
            }
        }
    }


    #[test]
    fn test_all() {
        for lang in &LANGS {
            let word_list = all(lang);
            
            // Check if the result is not empty
            assert!(!word_list.is_empty());

            // Check that the array is sorted
            let mut sorted_word_list = word_list.to_vec();
            sorted_word_list.sort_by_key(|s| UniCase::new(s.clone()));
            assert_eq!(word_list, &sorted_word_list[..], "Returned array should be sorted.");
        }
    }
}