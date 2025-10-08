use crate::solution::Solution;

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut ans = Vec::new();
        let char_pattern: Vec<char> = pattern.chars().collect();

        for query in queries {
            let mut curr_ptr = 0;
            let mut is_valid = true;

            for ch in query.chars() {
                if curr_ptr < char_pattern.len() && ch == char_pattern[curr_ptr] {
                    curr_ptr += 1;
                } else if ch.is_uppercase() {
                    // Extra uppercase letter that doesn't match pattern
                    is_valid = false;
                    break;
                }
                // Lowercase letters that don't match are okay (they're skipped)
            }

            // Check if we matched all pattern characters
            ans.push(is_valid && curr_ptr == char_pattern.len());
        }

        ans
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[derive(Debug, Clone)]
    struct TestCaseInput {
        queries: Vec<String>,
        pattern: String,
    }

    struct TestCase {
        input: TestCaseInput,
        output: Vec<bool>,
    }

    #[test]
    fn test_camel_match() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    queries: vec![
                        "FooBar".to_string(),
                        "FooBarTest".to_string(),
                        "FootBall".to_string(),
                        "FrameBuffer".to_string(),
                        "ForceFeedBack".to_string(),
                    ],
                    pattern: "FB".to_string(),
                },
                output: vec![true, false, true, true, false],
            },
            TestCase {
                input: TestCaseInput {
                    queries: vec![
                        "FooBar".to_string(),
                        "FooBarTest".to_string(),
                        "FootBall".to_string(),
                        "FrameBuffer".to_string(),
                        "ForceFeedBack".to_string(),
                    ],
                    pattern: "FoBa".to_string(),
                },
                output: vec![true, false, true, false, false],
            },
            TestCase {
                input: TestCaseInput {
                    queries: vec![
                        "FooBar".to_string(),
                        "FooBarTest".to_string(),
                        "FootBall".to_string(),
                        "FrameBuffer".to_string(),
                        "ForceFeedBack".to_string(),
                    ],
                    pattern: "FoBaT".to_string(),
                },
                output: vec![false, true, false, false, false],
            },
        ];

        for tc in test_cases {
            assert_eq!(
                Solution::camel_match(tc.input.queries.clone(), tc.input.pattern.clone()),
                tc.output,
                "Test: {:?}",
                tc.input
            )
        }
    }
}
