use crate::solution::Solution;

impl Solution {
    pub fn valid(str1: &str, str2: &str, k: usize) -> bool {
        if str1.len() % k > 0 || str2.len() % k > 0 {
            return false;
        } else {
            let base = &str1[0..k];
            return str1.replace(base, "").is_empty() && str2.replace(base, "").is_empty();
        }
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        for i in (1..=str1.len().min(str2.len())).rev() {
            if Self::valid(&str1, &str2, i) {
                return str1[0..i].to_string();
            }
        }

        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCase {
        input: (String, String),
        output: String,
    }

    #[test]
    fn test_gcd_of_strings() {
        let test_cases = vec![
            TestCase {
                input: ("ABCABC".to_string(), "ABC".to_string()),
                output: "ABC".to_string(),
            },
            TestCase {
                input: ("ABABAB".to_string(), "ABAB".to_string()),
                output: "AB".to_string(),
            },
            TestCase {
                input: ("LEET".to_string(), "CODE".to_string()),
                output: "".to_string(),
            },
            TestCase {
                input: ("ABCDEF".to_string(), "ABC".to_string()),
                output: "".to_string(),
            },
        ];

        for tc in test_cases {
            let result = Solution::gcd_of_strings(tc.input.0, tc.input.1);
            assert_eq!(result, tc.output);
        }
    }
}
