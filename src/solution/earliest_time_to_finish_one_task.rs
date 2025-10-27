use crate::solution::Solution;

impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        let mut min_sum = tasks[0][0] + tasks[0][1];
        for task in tasks.iter() {
            min_sum = min_sum.min(task[0] + task[1]);
        }
        return min_sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        tasks: Vec<Vec<i32>>,
        output: i32,
    }

    #[test]

    fn test_earliest_time() {
        let test_cases = vec![
            TestCase {
                tasks: vec![vec![1, 6], vec![2, 3]],
                output: 5,
            },
            TestCase {
                tasks: vec![vec![100, 100], vec![100, 100], vec![100, 100]],
                output: 200,
            },
        ];
        for tc in test_cases {
            assert_eq!(Solution::earliest_time(tc.tasks), tc.output);
        }
    }
}
