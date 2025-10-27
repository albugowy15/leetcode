use crate::solution::Solution;

use std::collections::HashSet;
impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        let mut sets: HashSet<i32> = HashSet::new();

        for i in 1..=n {
            sets.insert(i as i32);
        }

        for row in 0..n {
            let mut track_sets = sets.clone();
            for col in 0..n {
                let cell = matrix[row][col];
                track_sets.remove(&cell);
            }
            if !track_sets.is_empty() {
                return false;
            }
        }

        for row in 0..n {
            let mut track_sets = sets.clone();
            for col in 0..n {
                let cell = matrix[col][row];
                track_sets.remove(&cell);
            }
            if !track_sets.is_empty() {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        matrix: Vec<Vec<i32>>,
        output: bool,
    }

    #[test]

    fn test_check_valid() {
        let test_cases = vec![
            TestCase {
                matrix: vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]],
                output: true,
            },
            TestCase {
                matrix: vec![vec![1, 1, 1], vec![1, 2, 3], vec![1, 2, 3]],
                output: false,
            },
        ];
        for tc in test_cases {
            assert_eq!(Solution::check_valid(tc.matrix), tc.output);
        }
    }
}
