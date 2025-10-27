use crate::solution::Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut offset_idx = 0;
        let mut word_chars = word.chars().collect::<Vec<char>>();

        for (idx, val) in word_chars.iter().enumerate() {
            if *val == ch {
                offset_idx = idx;
                break;
            }
        }

        if offset_idx != 0 {
            let mut start_idx = 0;
            while start_idx < offset_idx {
                let temp = word_chars[start_idx].clone();
                word_chars[start_idx] = word_chars[offset_idx];
                word_chars[offset_idx] = temp;

                start_idx += 1;
                offset_idx -= 1;
            }
        }

        return word_chars.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCaseInput {
        word: String,
        ch: char,
    }

    struct TestCase {
        input: TestCaseInput,
        output: String,
    }

    #[test]
    fn test_reverse_prefix() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    word: "abcdefd".to_string(),
                    ch: 'd',
                },
                output: "dcbaefd".to_string(),
            },
            TestCase {
                input: TestCaseInput {
                    word: "xyxzxe".to_string(),
                    ch: 'z',
                },
                output: "zxyxxe".to_string(),
            },
            TestCase {
                input: TestCaseInput {
                    word: "abcd".to_string(),
                    ch: 'z',
                },
                output: "abcd".to_string(),
            },
            TestCase {
                input: TestCaseInput {
                    word: "abcdefd".to_string(),
                    ch: 'e',
                },
                output: "edcbafd".to_string(),
            },
        ];

        for tc in test_cases {
            assert_eq!(
                Solution::reverse_prefix(tc.input.word, tc.input.ch),
                tc.output
            );
        }
    }
}
