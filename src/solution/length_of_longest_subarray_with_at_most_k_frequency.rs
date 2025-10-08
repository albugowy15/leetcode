use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 1;

        for i in 0..nums.len() {
            let mut curr_len = 1;
            let mut freqs: HashMap<i32, i32> = HashMap::new();
            let mut j = i;
            for j in i..nums.len() {
                let num = nums[j];
                match freqs.get_mut(&num) {
                    Some(val) => {
                        let new_val = *val + 1;
                        if new_val <= k {
                            *val += 1;
                            curr_len += 1;
                        } else {
                            ans = ans.max(curr_len);
                        }
                    }
                    None => {
                        freqs.insert(num, 1);
                    }
                }
            }
        }
        return ans;
    }
}
