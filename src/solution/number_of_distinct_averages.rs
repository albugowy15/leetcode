use crate::solution::Solution;

use std::collections::HashSet;
impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut start_idx = 0;
        let mut end_idx = nums.len() - 1;
        let mut average_sets: HashSet<String> = HashSet::new();

        while start_idx < end_idx {
            let avg: f32 = (nums[start_idx] + nums[end_idx]) as f32 / 2.0;
            average_sets.insert(avg.to_string());
            start_idx += 1;
            end_idx -= 1;
        }

        return average_sets.len() as i32;
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
    fn test_distinct_averages() {
        let test_cases = vec![
            TestCase {
                input: vec![4, 1, 4, 0, 3, 5],
                output: 2,
            },
            TestCase {
                input: vec![1, 100],
                output: 1,
            },
            TestCase {
                input: vec![0, 0, 7, 2],
                output: 2,
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::distinct_averages(tc.input), tc.output);
        }
    }
}
