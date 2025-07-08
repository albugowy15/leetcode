use super::Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut prefix_sum = 0i64;
        let mut letters: Vec<char> = s.chars().collect();

        for idx in (0..letters.len()).rev() {
            prefix_sum += shifts[idx] as i64;
            let curr_char = letters[idx];
            let shift_amount = ((prefix_sum % 26) + 26) % 26;
            let char_index = (curr_char as u32 - 'a' as u32) as i64;
            let new_char_index = ((char_index + shift_amount) % 26) as u32;
            let new_char = char::from_u32('a' as u32 + new_char_index).unwrap();
            letters[idx] = new_char;
        }
        return letters.iter().collect::<String>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCaseInput {
        s: String,
        shifts: Vec<i32>,
    }

    struct TestCase {
        input: TestCaseInput,
        output: String,
    }

    #[test]
    fn test_shifting_letters() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    s: "abc".to_string(),
                    shifts: vec![3, 5, 9],
                },
                output: "rpl".to_string(),
            },
            TestCase {
                input: TestCaseInput {
                    s: "aaa".to_string(),
                    shifts: vec![1, 2, 3],
                },
                output: "gfd".to_string(),
            },
            TestCase {
                input: TestCaseInput {
                    s: "bad".to_string(),
                    shifts: vec![10, 20, 30],
                },
                output: "jyh".to_string(),
            },
        ];

        for test in test_cases {
            let result = Solution::shifting_letters(test.input.s, test.input.shifts);
            assert_eq!(result, test.output);
        }
    }
}
