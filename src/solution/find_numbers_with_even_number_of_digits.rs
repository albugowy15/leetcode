use crate::solution::Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for num in nums.into_iter() {
            let mut num_change = num;
            // edge case nums < 10
            if num < 10 {
                continue;
            }
            let mut divide_count = 0;

            while num_change >= 10 {
                divide_count += 1;
                num_change = num_change / 10;
            }
            if divide_count % 2 == 1 {
                ans += 1;
            } else {
                continue;
            }
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        input: Vec<i32>,
        output: i32,
    }
    #[test]
    fn test_find_numbers() {
        let test_cases = [
            TestCase {
                input: vec![12, 345, 2, 6, 7896],
                output: 2,
            },
            TestCase {
                input: vec![555, 901, 482, 1771],
                output: 1,
            },
        ];

        for tc in test_cases.into_iter() {
            let result = Solution::find_numbers(tc.input);
            assert_eq!(result, tc.output);
        }
    }
}
