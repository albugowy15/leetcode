use crate::solution::Solution;
// Problem: https://leetcode.com/problems/rotate-array

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let pivot: usize = k as usize % nums.len();
        nums.reverse();
        nums[0..pivot].reverse();
        nums[pivot..].reverse();
    }
}

#[cfg(test)]
mod tests {
    struct RotateArrayTestCase {
        nums: Vec<i32>,
        k: i32,
        result: Vec<i32>,
    }
    #[test]
    fn test_rotate_array() {
        let mut test_cases = [
            RotateArrayTestCase {
                nums: vec![1, 2, 3, 4, 5, 6, 7],
                k: 3,
                result: vec![5, 6, 7, 1, 2, 3, 4],
            },
            RotateArrayTestCase {
                nums: vec![-1, -100, 3, 99],
                k: 2,
                result: vec![3, 99, -1, -100],
            },
        ];

        for test in test_cases.iter_mut() {
            super::Solution::rotate(&mut test.nums, test.k);
            assert_eq!(test.result, test.nums);
        }
    }
}
