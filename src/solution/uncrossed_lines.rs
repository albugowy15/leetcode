use std::vec;

use super::Solution;

impl Solution {
    fn solve(
        i: usize,
        j: usize,
        nums1: &Vec<i32>,
        nums2: &Vec<i32>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if i <= 0 || j <= 0 {
            return 0;
        }

        if memo[i as usize][j] != -1 {
            return memo[i][j];
        }

        if nums1[i - 1] == nums2[j - 1] {
            memo[i][j] = 1 + Self::solve(i - 1, j - 1, nums1, nums2, memo);
        } else {
            memo[i][j] = std::cmp::max(
                Self::solve(i, j - 1, nums1, nums2, memo),
                Self::solve(i - 1, j, nums1, nums2, memo),
            )
        }
        memo[i][j]
    }
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len();
        let n2 = nums2.len();

        let mut memo = vec![vec![-1; n2 + 1]; n1 + 1];

        Self::solve(n1, n2, &nums1, &nums2, &mut memo)
    }
}

#[test]
fn test() {
    let (nums_test_1, num_test_2) = (vec![1, 4, 2], vec![1, 2, 4]);
    assert_eq!(Solution::max_uncrossed_lines(nums_test_1, num_test_2), 2);

    let (nums_test_1, num_test_2) = (vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]);
    assert_eq!(Solution::max_uncrossed_lines(nums_test_1, num_test_2), 3);

    let (nums_test_1, num_test_2) = (vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]);
    assert_eq!(Solution::max_uncrossed_lines(nums_test_1, num_test_2), 2);
}
