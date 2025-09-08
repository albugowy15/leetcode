use crate::solution::Solution;

impl Solution {
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut middle_ptr = (n + 1) / 2;
        let mut start_ptr = 0;

        while middle_ptr <= n - 1 {
            if 2 * nums[start_ptr] <= nums[middle_ptr] {
                start_ptr += 1;
            }
            middle_ptr += 1;
        }

        return start_ptr as i32 * 2;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestCase {
        input: Vec<i32>,
        output: i32,
    }

    #[test]
    fn test_max_num_of_marked_indices() {
        let test_cases = [
            TestCase {
                input: vec![3, 5, 2, 4], // 2 3 4 5
                output: 2,
            },
            TestCase {
                input: vec![9, 2, 5, 4],
                output: 4,
            },
            TestCase {
                input: vec![7, 6, 8],
                output: 0,
            },
            TestCase {
                input: vec![
                    57, 40, 57, 51, 90, 51, 68, 100, 24, 39, 11, 85, 2, 22, 67, 29, 74, 82, 10, 96,
                    14, 35, 25, 76, 26, 54, 29, 44, 63, 49, 73, 50, 95, 89, 43, 62, 24, 88, 88, 36,
                    6, 16, 14, 2, 42, 42, 60, 25, 4, 58, 23, 22, 27, 26, 3, 79, 64, 20, 92,
                ],
                output: 58,
            },
        ];

        for tc in test_cases {
            let result = Solution::max_num_of_marked_indices(tc.input);
            assert_eq!(result, tc.output);
        }
    }
}
