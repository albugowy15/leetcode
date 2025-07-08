use super::Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut counter = 0;
        let bits: Vec<char> = s.chars().collect();
        let (mut zero_counter, mut one_counter) = (0, 0);

        let mut left = 0;
        for right in 0..bits.len() {
            let curr_char = bits[right];
            if curr_char == '0' {
                zero_counter += 1;
            } else {
                one_counter += 1;
            }

            while zero_counter > k && one_counter > k {
                if bits[left] == '0' {
                    zero_counter -= 1;
                } else {
                    one_counter -= 1;
                }
                left += 1;
            }
            counter += (right as i32) - left as i32 + 1;
        }
        return counter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCaseInput {
        s: String,
        k: i32,
    }
    struct TestCase {
        input: TestCaseInput,
        output: i32,
    }

    #[test]
    fn test_count_k_constraint_substrings() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    s: String::from("10101"),
                    k: 1,
                },
                output: 12,
            },
            TestCase {
                input: TestCaseInput {
                    s: String::from("1010101"),
                    k: 2,
                },
                output: 25,
            },
            TestCase {
                input: TestCaseInput {
                    s: String::from("11111"),
                    k: 1,
                },
                output: 15,
            },
        ];

        for test in test_cases {
            let result = Solution::count_k_constraint_substrings(test.input.s, test.input.k);
            assert_eq!(result, test.output);
        }
    }
}
