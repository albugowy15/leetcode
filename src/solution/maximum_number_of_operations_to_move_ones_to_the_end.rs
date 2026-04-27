struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut one_count = 0;
        let mut ans = 0;

        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '0' {
                while (i + 1) < chars.len() && chars[i + 1] == '0' {
                    i += 1;
                }
                ans += one_count;
            } else {
                one_count += 1;
            }
            i += 1;
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: String,
        output: i32,
    }

    #[test]
    fn test_max_operations() {
        let test_cases = vec![
            TestCase {
                input: String::from("1001101"),
                output: 4,
            },
            TestCase {
                input: String::from("00111"),
                output: 0,
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::max_operations(tc.input), tc.output);
        }
    }
}
