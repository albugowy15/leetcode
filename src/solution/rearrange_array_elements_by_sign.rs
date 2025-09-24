use crate::solution::Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let mut pos_idx = 0;
        let mut neg_idx = 1;

        for num in nums {
            if num >= 0 {
                ans[pos_idx] = num;
                pos_idx += 2; // next positive slot
            } else {
                ans[neg_idx] = num;
                neg_idx += 2; // next negative slot
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCase {
        nums: Vec<i32>,
        expected: Vec<i32>,
    }

    #[test]
    fn test_rearrange_array() {
        let test_cases = vec![
            TestCase {
                nums: vec![3, 1, -2, -5, 2, -4],
                expected: vec![3, -2, 1, -5, 2, -4],
            },
            TestCase {
                nums: vec![-1, 1],
                expected: vec![1, -1],
            },
            TestCase {
                nums: vec![1, -1],
                expected: vec![1, -1],
            },
            TestCase {
                nums: vec![4, -3, 2, -1],
                expected: vec![4, -3, 2, -1],
            },
        ];

        for tc in test_cases {
            let result = Solution::rearrange_array(tc.nums.clone());
            assert_eq!(result, tc.expected, "Failed for input: {:?}", tc.nums);
        }
    }
}
