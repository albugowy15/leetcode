use crate::solution::Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut max_distance = 0;

        // Find all water cells (0) and calculate distance to nearest land (1)
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 0 {
                    let distance = Self::find_nearest_land_distance(&grid, row, col);
                    max_distance = std::cmp::max(distance, max_distance);
                }
            }
        }

        if max_distance == 0 {
            -1
        } else {
            max_distance
        }
    }

    // BFS to find distance to nearest land
    fn find_nearest_land_distance(grid: &Vec<Vec<i32>>, start_row: usize, start_col: usize) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut queue = VecDeque::new();

        // Start BFS from the water cell
        queue.push_back((start_row, start_col, 0));
        visited[start_row][start_col] = true;

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // right, left, down, up

        while let Some((row, col, distance)) = queue.pop_front() {
            // If we found land, return the distance
            if grid[row][col] == 1 {
                return distance;
            }

            // Explore all 4 directions
            for (dr, dc) in directions.iter() {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                // Check bounds
                if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if !visited[new_row][new_col] {
                        visited[new_row][new_col] = true;
                        queue.push_back((new_row, new_col, distance + 1));
                    }
                }
            }
        }

        0 // No land found (shouldn't happen in valid input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_distance() {
        let grid1 = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(Solution::max_distance(grid1.clone()), 2);
        assert_eq!(Solution::max_distance(grid1), 2);

        let grid2 = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::max_distance(grid2.clone()), 4);
        assert_eq!(Solution::max_distance(grid2), 4);

        let grid3 = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::max_distance(grid3.clone()), -1);
        assert_eq!(Solution::max_distance(grid3), -1);

        let grid4 = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::max_distance(grid4.clone()), -1);
        assert_eq!(Solution::max_distance(grid4), -1);
    }
}
