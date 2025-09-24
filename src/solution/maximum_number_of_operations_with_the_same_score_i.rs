use crate::solution::Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut left_ptr = 0;
        let mut right_ptr = 1;

        let prev_sum = nums[left_ptr] + nums[right_ptr];
        let mut ans = 0;
        while right_ptr < nums.len() {
            if nums[left_ptr] + nums[right_ptr] == prev_sum {
                ans += 1;
                left_ptr = right_ptr + 1;
                right_ptr = left_ptr + 1;
            } else {
                break;
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
    fn test_max_operations() {
        let test_cases = vec![
            TestCase {
                input: vec![3, 2, 1, 4, 5],
                output: 2,
            },
            TestCase {
                input: vec![1, 5, 3, 3, 4, 1, 3, 2, 2, 3],
                output: 2,
            },
            TestCase {
                input: vec![5, 3],
                output: 1,
            },
        ];

        for case in test_cases {
            println!("Input: {:?}, Expected Output: {}", case.input, case.output);
            assert_eq!(Solution::max_operations(case.input), case.output);
        }
    }
}
