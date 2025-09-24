use crate::solution::Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|k| k[0]);
        let mut merged: Vec<Vec<i32>> = Vec::new();

        for interval in intervals {
            if merged.is_empty() || merged.last().unwrap()[1] < interval[0] {
                merged.push(interval);
            } else {
                merged.last_mut().unwrap()[1] = merged.last().unwrap()[1].max(interval[1]);
            }
        }

        return merged;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<Vec<i32>>,
        output: Vec<Vec<i32>>,
    }

    #[test]
    fn test_merge() {
        let test_cases = vec![
            TestCase {
                input: vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
                output: vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            },
            TestCase {
                input: vec![vec![1, 4], vec![4, 5]],
                output: vec![vec![1, 5]],
            },
            TestCase {
                input: vec![vec![4, 7], vec![1, 4]],
                output: vec![vec![1, 7]],
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::merge(tc.input), tc.output);
        }
    }
}
