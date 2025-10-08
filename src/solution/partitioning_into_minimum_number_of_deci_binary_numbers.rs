use crate::solution::Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut max_digit = 0;

        for ch in n.chars() {
            let ch_num = ch.to_digit(10).unwrap();
            max_digit = max_digit.max(ch_num);
        }

        return max_digit.try_into().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    struct TestCase {
        input: String,
        expected: i32,
    }

    #[test]
    fn test_min_partitions() {
        let test_cases = vec![
            TestCase {
                input: String::from("32"),
                expected: 3,
            },
            TestCase {
                input: String::from("82734"),
                expected: 8,
            },
            TestCase {
                input: String::from("27346209830709182346"),
                expected: 9,
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::min_partitions(tc.input), tc.expected);
        }
    }
}
