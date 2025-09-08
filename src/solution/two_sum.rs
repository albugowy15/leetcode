use std::{collections::HashMap, vec};

use super::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in (0..nums.len()).step_by(1) {
            let complement = target - nums[i];
            if map.contains_key(&complement) {
                return vec![map[&complement], i as i32];
            }
            map.insert(nums[i], i as i32);
        }

        vec![]
    }
}

#[test]
fn test() {
    // testcase 1
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);

    // testcase 2
    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);

    // testcase 3
    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}
