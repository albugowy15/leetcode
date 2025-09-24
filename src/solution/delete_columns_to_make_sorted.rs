use crate::solution::Solution;

use std::collections::HashSet;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let grids: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();

        let mut unordered_cols: HashSet<usize> = HashSet::new();
        for row in 0..grids.len() {
            for col in 0..grids[row].len() {
                let prev_row_idx = if row == 0 { 0 } else { row - 1 };
                let prev = grids[prev_row_idx][col];
                let curr = grids[row][col];

                match prev.cmp(&curr) {
                    std::cmp::Ordering::Greater => {
                        unordered_cols.insert(col);
                    }
                    _ => continue,
                }
            }
        }
        return unordered_cols.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<String>,
        output: i32,
    }

    #[test]
    fn test_min_deletion_size() {
        let test_cases = vec![
            TestCase {
                input: vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()],
                output: 1,
            },
            TestCase {
                input: vec!["a".to_string(), "b".to_string()],
                output: 0,
            },
            TestCase {
                input: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                output: 0,
            },
            TestCase {
                input: vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()],
                output: 3,
            },
        ];

        for tc in test_cases {
            println!("Input: {:?}, Expected Output: {}", tc.input, tc.output);
            assert_eq!(Solution::min_deletion_size(tc.input), tc.output);
        }
    }
}
