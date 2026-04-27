use crate::solution::Solution;

use std::collections::HashSet;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let result_set: HashSet<i32> = HashSet::from_iter(nums);
        if result_set.contains(&0) {
            return result_set.len() as i32 - 1;
        } else {
            return result_set.len() as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        nums: Vec<i32>,
        output: i32,
    }

    #[test]
    fn test_minimum_operations() {
        let test_cases = vec![
            TestCase {
                nums: vec![1, 5, 0, 3, 5],
                output: 3,
            },
            TestCase {
                nums: vec![1, 2, 3, 4, 5],
                // 1 2 3 4 5
                // 0 1 2 3 4
                // 0 0 1 2 3
                // 0 0 0 1 2
                // 0 0 0 0 1
                // 0 0 0 0 0
                output: 5,
            },
            TestCase {
                nums: vec![0],
                output: 0,
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::minimum_operations(tc.nums), tc.output);
        }
    }
}
