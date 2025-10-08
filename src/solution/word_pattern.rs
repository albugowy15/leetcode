use crate::solution::Solution;

use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut letter_to_word: HashMap<char, &str> = HashMap::new();
        let mut word_to_letter: HashMap<&str, char> = HashMap::new();
        let letters: Vec<char> = pattern.chars().collect();
        let words: Vec<&str> = s.split_whitespace().collect();

        let letters_len = letters.len();
        let words_len = words.len();

        if letters_len != words_len {
            return false;
        }

        for i in 0..letters_len {
            let letter = letters[i];
            let word = words[i];
            match word_to_letter.get(word) {
                Some(prev_letter) => {
                    if *prev_letter != letter {
                        return false;
                    }
                }
                None => {
                    word_to_letter.insert(word, letter);
                }
            }
            match letter_to_word.get(&letter) {
                Some(prev_word) => {
                    if *prev_word != word {
                        return false;
                    }
                }
                None => {
                    letter_to_word.insert(letter, word);
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCaseInput {
        pattern: String,
        s: String,
    }

    struct TestCase {
        input: TestCaseInput,
        output: bool,
    }

    #[test]
    fn test_word_pattern() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    pattern: "abba".to_string(),
                    s: "dog cat cat dog".to_string(),
                },
                output: true,
            },
            TestCase {
                input: TestCaseInput {
                    pattern: "abba".to_string(),
                    s: "dog cat cat fish".to_string(),
                },
                output: false,
            },
            TestCase {
                input: TestCaseInput {
                    pattern: "abba".to_string(),
                    s: "dog dog dog dog".to_string(),
                },
                output: false,
            },
            TestCase {
                input: TestCaseInput {
                    pattern: "aaaa".to_string(),
                    s: "dog cat cat dog".to_string(),
                },
                output: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(
                Solution::word_pattern(tc.input.pattern, tc.input.s),
                tc.output
            );
        }
    }
}
