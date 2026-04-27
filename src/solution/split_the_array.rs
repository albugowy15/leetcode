use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut freq_counter: HashMap<i32, i32> = HashMap::new();

        for num in nums.iter() {
            let counter = freq_counter
                .entry(*num)
                .and_modify(|val| *val += 1)
                .or_insert(1);
            if *counter > 2 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase {
        nums: Vec<i32>,
        output: bool,
    }
    #[test]
    fn test_is_possible_to_split() {
        let test_cases = vec![
            TestCase {
                nums: vec![1, 1, 2, 2, 3, 4],
                output: true,
            },
            TestCase {
                nums: vec![1, 1, 1, 1],
                output: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(Solution::is_possible_to_split(tc.nums), tc.output);
        }
    }
}
