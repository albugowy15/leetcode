use crate::solution::Solution;

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut word_freq: HashMap<&str, i32> = HashMap::with_capacity(words.len());

        words.iter().for_each(|word| {
            *word_freq.entry(word).or_insert(0) += 1;
        });

        let mut sorted: Vec<(&str, i32)> = Vec::with_capacity(word_freq.len());

        word_freq.iter().for_each(|(word, freq)| {
            sorted.push((word, *freq));
        });

        sorted.sort_by(|a, b| {
            let freq_diff = a.1 - b.1;
            if freq_diff == 0 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        sorted[0..k as usize]
            .iter()
            .map(|(word, _)| word.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCase {
        input: (Vec<String>, i32),
        output: Vec<String>,
    }

    #[test]
    fn test_top_k_frequent() {
        let test_cases = vec![
            TestCase {
                input: (
                    vec![
                        "i".to_string(),
                        "love".to_string(),
                        "leetcode".to_string(),
                        "i".to_string(),
                        "love".to_string(),
                        "coding".to_string(),
                    ],
                    2,
                ),
                output: vec!["i".to_string(), "love".to_string()],
            },
            TestCase {
                input: (
                    vec![
                        "the".to_string(),
                        "day".to_string(),
                        "is".to_string(),
                        "sunny".to_string(),
                        "the".to_string(),
                        "the".to_string(),
                        "the".to_string(),
                        "sunny".to_string(),
                        "is".to_string(),
                        "is".to_string(),
                    ],
                    4,
                ),
                output: vec![
                    "the".to_string(),
                    "is".to_string(),
                    "sunny".to_string(),
                    "day".to_string(),
                ],
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::top_k_frequent(tc.input.0, tc.input.1), tc.output);
        }
    }
}
