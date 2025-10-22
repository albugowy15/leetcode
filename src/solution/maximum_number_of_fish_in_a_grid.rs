use crate::solution::Solution;

fn walk(row: usize, col: usize, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    if row >= rows || col >= cols || visited[row][col] || grid[row][col] == 0 {
        return 0;
    }
    visited[row][col] = true;
    let mut sum = grid[row][col];
    if row > 0 {
        sum += walk(row - 1, col, grid, visited);
    }
    if row + 1 < rows {
        sum += walk(row + 1, col, grid, visited);
    }
    if col > 0 {
        sum += walk(row, col - 1, grid, visited);
    }
    if col + 1 < cols {
        sum += walk(row, col + 1, grid, visited);
    }
    sum
}

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
        let mut max_sum = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != 0 && !visited[r][c] {
                    max_sum = max_sum.max(walk(r, c, &grid, &mut visited));
                }
            }
        }

        return max_sum;
    }
}

// check every cell
// if we get cell with 0 -> continue to next cell
// if we get cell with not 0 -> process
//      sum the cell value
//      go to right -> add to sum
//      go to left -> add to sum
//      go to right -> add to sum
//      go to bottom -> add to sum
//  track the maximum sum
//
#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        grid: Vec<Vec<i32>>,
        expected: i32,
    }
    #[test]
    fn test_find_max_fish() {
        let test_cases = vec![
            TestCase {
                grid: vec![
                    vec![0, 2, 1, 0],
                    vec![4, 0, 0, 3],
                    vec![1, 0, 0, 4],
                    vec![0, 3, 2, 0],
                ],
                expected: 7,
            },
            TestCase {
                grid: vec![
                    vec![1, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 1],
                ],
                expected: 1,
            },
            TestCase {
                grid: vec![vec![4]],
                expected: 4,
            },
            TestCase {
                grid: vec![vec![9, 10]],
                expected: 19,
            },
        ];

        for (idx, tc) in test_cases.into_iter().enumerate() {
            assert_eq!(
                Solution::find_max_fish(tc.grid),
                tc.expected,
                "Testing {}:",
                idx + 1
            )
        }
    }
}
