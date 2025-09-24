use crate::solution::Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(2 * n as usize);

        let mut left_ptr: usize = 0;
        let mut right_ptr: usize = n as usize;
        while left_ptr < n as usize {
            ans.push(nums[left_ptr]);
            ans.push(nums[right_ptr]);
            left_ptr += 1;
            right_ptr += 1;
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCase {
        input: (Vec<i32>, i32),
        expect: Vec<i32>,
    }

    #[test]
    fn test_shuffle() {
        let test_cases = vec![
            TestCase {
                input: (vec![2, 5, 1, 3, 4, 7], 3),
                expect: vec![2, 3, 5, 4, 1, 7],
            },
            TestCase {
                input: (vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
                expect: vec![1, 4, 2, 3, 3, 2, 4, 1],
            },
            TestCase {
                input: (vec![1, 1, 2, 2], 2),
                expect: vec![1, 2, 1, 2],
            },
        ];

        for tc in test_cases {
            let ans = Solution::shuffle(tc.input.0, tc.input.1);
            assert_eq!(ans, tc.expect);
        }
    }
}
