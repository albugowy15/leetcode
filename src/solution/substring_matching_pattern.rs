use crate::solution::Solution;

impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        if let Some(x) = p.find('*') {
            let b = &p[..x];
            let e = &p[x + 1..];
            if let (Some(i), Some(j)) = (s.find(b), s.rfind(e)) {
                return i + b.len() <= j;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCaseInput {
        s: String,
        p: String,
    }

    struct TestCase {
        input: TestCaseInput,
        expected: bool,
    }

    #[test]
    fn test_has_match() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    s: String::from("leetcode"),
                    p: String::from("ee*e"),
                },
                expected: true,
            },
            TestCase {
                input: TestCaseInput {
                    s: String::from("car"),
                    p: String::from("c*v"),
                },
                expected: false,
            },
            TestCase {
                input: TestCaseInput {
                    s: String::from("luck"),
                    p: String::from("u*"),
                },
                expected: true,
            },
        ];

        for (idx, tc) in test_cases.into_iter().enumerate() {
            assert_eq!(
                Solution::has_match(tc.input.s, tc.input.p),
                tc.expected,
                "Test Case #{}",
                idx + 1
            );
        }
    }
}
