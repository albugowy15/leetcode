use crate::solution::Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut maps: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::with_capacity(nums.len());

        nums.into_iter().for_each(|num| {
            maps.entry(num).and_modify(|val| *val += 1).or_insert(1);
        });

        maps.iter().for_each(|(key, val)| {
            if *val == 1 && !maps.contains_key(&(key - 1)) && !maps.contains_key(&(key + 1)) {
                result.push(*key);
            }
        });
        return result;
    }
}

#[cfg(test)]
mod tests {
    struct TestCase {
        input: Vec<i32>,
        expected: Vec<i32>,
    }

    use super::*;

    #[test]
    fn test_find_lonely() {
        let test_cases = vec![
            TestCase {
                input: vec![10, 6, 5, 8],
                expected: vec![10, 8],
            },
            TestCase {
                input: vec![1, 3, 5, 3],
                expected: vec![1, 5],
            },
        ];

        for (idx, mut tc) in test_cases.into_iter().enumerate() {
            assert_eq!(
                Solution::find_lonely(tc.input).sort(),
                tc.expected.sort(),
                "Test case #{}",
                idx + 1
            );
        }
    }
}
