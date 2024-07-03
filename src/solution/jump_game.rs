struct Solution;
// Problem: https://leetcode.com/problems/jump-game

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut final_pos = len - 1;
        for pos in (0..len).rev() {
            let diff = (final_pos - pos) as i32;
            if nums[pos] >= diff {
                final_pos = pos;
            }
        }
        final_pos == 0
    }
}

#[cfg(test)]
mod tests {
    struct JumpGameTestCase {
        nums: Vec<i32>,
        result: bool,
    }
    #[test]
    fn test_jump_game() {
        let test_cases = [
            JumpGameTestCase {
                nums: vec![2, 3, 1, 1, 4],
                result: true,
            },
            JumpGameTestCase {
                nums: vec![3, 2, 1, 0, 4],
                result: false,
            },
            JumpGameTestCase {
                nums: vec![3],
                result: true,
            },
            JumpGameTestCase {
                nums: vec![3, 4],
                result: true,
            },
            JumpGameTestCase {
                nums: vec![0, 4],
                result: false,
            },
            JumpGameTestCase {
                nums: vec![2, 0, 0],
                result: true,
            },
        ];

        for test in test_cases {
            assert_eq!(test.result, super::Solution::can_jump(test.nums));
        }
    }
}
