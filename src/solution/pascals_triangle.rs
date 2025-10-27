use crate::solution::Solution;

impl Solution {
    fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as usize;
        let mut row: Vec<i32> = vec![1; n + 1];

        for i in 1..=n {
            for j in (1..i).rev() {
                row[j] = row[j] + row[j - 1];
            }
        }
        return row;
    }

    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        for i in 0..num_rows {
            ans.push(Solution::get_row(i));
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        num_rows: i32,
        output: Vec<Vec<i32>>,
    }

    #[test]
    fn test_generate() {
        let test_cases = vec![
            TestCase {
                num_rows: 5,
                output: vec![
                    vec![1],
                    vec![1, 1],
                    vec![1, 2, 1],
                    vec![1, 3, 3, 1],
                    vec![1, 4, 6, 4, 1],
                ],
            },
            TestCase {
                num_rows: 1,
                output: vec![vec![1]],
            },
            TestCase {
                num_rows: 2,
                output: vec![vec![1], vec![1, 1]],
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::generate(tc.num_rows), tc.output);
        }
    }
}
