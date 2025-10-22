use crate::solution::Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (mut max_counter, mut counter) = (2, 2);
        for j in 2..nums.len() {
            if nums[j] == nums[j - 1] + nums[j - 2] {
                counter += 1;
            } else {
                counter = 2;
            }
            max_counter = max_counter.max(counter);
        }
        return max_counter;
    }
}

#[cfg(test)]
mod tests {
    struct TestCase {
        input: Vec<i32>,
        expected: i32,
    }

    use super::*;

    #[test]
    fn test_longest_subarray() {
        let test_cases = vec![
            TestCase {
                input: vec![1, 1, 1, 1, 2, 3, 5, 1],
                expected: 5,
            },
            TestCase {
                input: vec![5, 2, 7, 9, 16],
                expected: 5,
            },
            TestCase {
                input: vec![1000000000, 1000000000, 1000000000],
                expected: 2,
            },
            TestCase {
                input: vec![3, 1, 4, 5, 3, 1, 4, 3, 1, 4],
                expected: 4,
            },
        ];

        for (idx, tc) in test_cases.into_iter().enumerate() {
            assert_eq!(
                Solution::longest_subarray(tc.input),
                tc.expected,
                "Test case #{}",
                idx + 1
            );
        }
    }
}
