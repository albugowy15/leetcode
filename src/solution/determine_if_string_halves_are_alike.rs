use crate::solution::Solution;

use std::collections::HashSet;
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let vocals: HashSet<char> =
            HashSet::from(['a', 'i', 'u', 'e', 'o', 'A', 'I', 'U', 'E', 'O']);
        let mid: usize = s.len() / 2;
        let mut counter = 0;

        for (idx, ch) in s.chars().enumerate() {
            if vocals.contains(&ch) {
                if idx < mid {
                    counter += 1;
                } else {
                    counter -= 1;
                }
            }
        }

        return counter == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        s: String,
        output: bool,
    }

    #[test]
    fn test_halves_are_alike() {
        let test_cases = vec![
            TestCase {
                s: String::from("book"),
                output: true,
            },
            TestCase {
                s: String::from("textbook"),
                output: false,
            },
            TestCase {
                s: String::from("BoOk"),
                output: true,
            },
            TestCase {
                s: String::from("AaaaiIaaAaIi"),
                output: true,
            },
            TestCase {
                s: String::from("AbCdEfGh"),
                output: true,
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::halves_are_alike(tc.s), tc.output);
        }
    }
}
