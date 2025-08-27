use crate::solution::Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 {
            return n;
        }
        return 2 * n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: i32,
        output: i32,
    }

    #[test]
    fn smallest_even_multiple() {
        let test_cases = [
            TestCase {
                input: 5,
                output: 10,
            },
            TestCase {
                input: 6,
                output: 6,
            },
            TestCase {
                input: 7,
                output: 14,
            },
        ];

        for tc in test_cases {
            let result = Solution::smallest_even_multiple(tc.input);
            assert_eq!(result, tc.output);
        }
    }
}
