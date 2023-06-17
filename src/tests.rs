#[cfg(feature = "all")]
#[cfg(test)]
mod Tests {
    use super::*;

    #[test]
    fn select_random_test() {
        let array = &["1", "2", "3", "4", "5"];
        // All options from array should be pushed into this set.
        // This ensures that the random selection function
        // is capable of selecting all available items from array.
        let mut set = ["0"; 5];
        (0..500000).for_each(|_| {
            let choice = select_random(array);
            set[match choice {
                "1" => 0,
                "2" => 1,
                "3" => 2,
                "4" => 3,
                "5" => 4,
                _ => panic!("Random choice is an unexpected value"),
            }] = choice;
            assert!(!choice.is_empty(), "Random choice should not be empty");
            assert!(
                array.contains(&choice),
                "Random choice should be from input array"
            );
        });
        assert_eq!(
            set.len(),
            array.len(),
            "Set length should match array length"
        );
        array.iter().for_each(|item| {
            assert!(set.contains(item), "Set should contain array item");
        });
    }

    #[test]
    fn gen_test() {
        let mut word_list = [""; 50];
        (0..50).for_each(|i| {
            let word = gen(Lang::En).unwrap();
            assert!(
                !word_list.contains(&word),
                "List of up to 50 previously generated words contains duplicates."
            );
            assert!(!word.is_empty(), "generated word should not be empty");
            word_list[i] = word;
        })
    }

    #[test]
    fn gen_starts_with_test() {
        ('a'..='z').chain('A'..='Z').for_each(|character| {
            (0..5000).for_each(|_| {
                let word = gen_starts_with(character, Lang::En).expect("character is not alphabetic");
                let character = character.to_ascii_lowercase();
                assert!(!word.is_empty(), "word should not be empty");
                assert!(
                    word.starts_with(character),
                    "word should start with {character}"
                )
            })
        })
    }

    #[test]
    fn all_starts_with_test() {
        ('a'..='z').chain('A'..'Z').for_each(|character| {
            let word_list = all_starts_with(character, Lang::En).expect("character is not alphabetic");
            word_list.iter().for_each(|word| {
                let character = character.to_ascii_lowercase();
                assert!(!word.is_empty(), "word should not be empty");
                assert!(
                    word.starts_with(character),
                    "word should start with {character}"
                );
            })
        })
    }

    #[test]
    fn gen_len_test() {
        (2..16).for_each(|length| {
            (0..5000).for_each(|_| {
                let word = gen_len(length, Lang::En).unwrap();
                assert!(!word.is_empty(), "word should not be empty in word list");
                assert!(
                    word.chars().all(|c| ('a'..='z').contains(&c)),
                    "word should only contain lowercase alphabetical characters"
                );
                assert_eq!(
                    word.len(),
                    length,
                    "word should be {length} characters in length"
                );
            })
        })
    }

    #[test]
    fn all_len_test() {
        (2..16).for_each(|length| {
            let word_list = all_len(length, Lang::En).unwrap();
            word_list.iter().for_each(|word| {
                assert!(!word.is_empty(), "word should not be empty in word list");
                assert!(
                    word.chars().all(|c| ('a'..='z').contains(&c)),
                    "word should only contain lowercase alphabetical characters"
                );
                assert_eq!(
                    word.len(),
                    length,
                    "word should be {length} characters in length"
                );
            })
        })
    }

    #[test]
    fn gen_len_starts_with_test() {
        (2..16).for_each(|length| {
            ('a'..='z').for_each(|character| match (length, character) {
                (2, 'c') | (2, 'v') | (15, 'y') => (),
                _ => {
                    let word = gen_len_starts_with(length, character, Lang::En).unwrap();
                    assert!(!word.is_empty(), "{word} should not be empty in word list");
                    assert!(
                        word.chars().all(|c| ('a'..='z').contains(&c)),
                        "{word} should only contain lowercase alphabetical characters"
                    );
                    assert_eq!(
                        word.len(),
                        length,
                        "{word} should be {length} characters in length"
                    );
                    assert!(
                        word.starts_with(character),
                        "{word} should start with {character}"
                    )
                }
            })
        })
    }

    #[test]
    fn all_len_starts_with_test() {
        (2..16).for_each(|length| {
            ('a'..='z').for_each(|character| match (length, character) {
                (2, 'c') | (2, 'v') | (15, 'y') => {
                    assert_eq!(all_len_starts_with(length, character, Lang::En), None)
                }
                _ => {
                    let word_list = all_len_starts_with(length, character, Lang::En).unwrap();
                    word_list.iter().for_each(|word| {
                        assert!(!word.is_empty(), "word should not be empty in word list");
                        assert!(
                            word.chars().all(|c| ('a'..='z').contains(&c)),
                            "word should only contain lowercase alphabetical characters"
                        );
                        assert_eq!(
                            word.len(),
                            length,
                            "word should be {length} characters in length"
                        );
                        assert!(
                            word.starts_with(character),
                            "word should start with {character}"
                        )
                    })
                }
            })
        })
    }

    #[test]
    fn all_test() {
        let word_list = all(Lang::En).unwrap();
        word_list.iter().for_each(|word| {
            assert!(!word.is_empty(), "word should not be empty in word list");
            assert!(
                word.chars().all(|c| ('a'..='z').contains(&c)),
                "word should only contain lowercase alphabetical characters"
            );
        })
    }

    #[test]
    fn all_indexes_from_first_char() {
        let word_list = all(Lang::En).unwrap();
        ('a'..='z').for_each(|c| {
            let _ = word_list.iter().position(|w| w.starts_with(c)).expect(
                "words starting with any alphabetic character 
                    should be represented in words array",
            );
        });
    }
}