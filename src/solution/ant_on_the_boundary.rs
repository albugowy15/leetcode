use crate::solution::Solution;

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut ans = 0;

        nums.iter().for_each(|num| {
            sum += num;
            if sum == 0 {
                ans += 1;
            }
        });
        return ans;
    }
}

// just sum all element
// during the loop if sum is equal to zero, increment the ans
//
#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCase {
        nums: Vec<i32>,
        output: i32,
    }

    #[test]
    fn test_return_to_boundary_count() {
        let test_cases = vec![
            TestCase {
                nums: vec![2, 3, -5],
                output: 1,
            },
            TestCase {
                nums: vec![3, 2, -3, -4],
                output: 0,
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::return_to_boundary_count(tc.nums), tc.output);
        }
    }
}
