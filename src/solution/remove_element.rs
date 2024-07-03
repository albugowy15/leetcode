struct Solution;
// Problem: https://leetcode.com/problems/remove-element/?envType=study-plan-v2&envId=top-interview-150

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        return k as i32;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_remove_element() {
        // tc 1
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let mut expected_nums = vec![2, 2];

        let k = super::Solution::remove_element(&mut nums, val);
        assert_eq!(expected_nums.len(), k as usize);
        let mut clean_nums = nums[0..k as usize].to_vec();
        clean_nums.sort();
        expected_nums.sort();

        assert_eq!(clean_nums, expected_nums);

        // tc 2
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let mut expected_nums = vec![0, 1, 4, 0, 3];

        let k = super::Solution::remove_element(&mut nums, val);
        assert_eq!(expected_nums.len(), k as usize);
        let mut clean_nums = nums[0..k as usize].to_vec();
        clean_nums.sort();
        expected_nums.sort();

        assert_eq!(clean_nums, expected_nums);

        // tc 3
        let mut nums = vec![3];
        let val = 2;
        let mut expected_nums = vec![3];

        let k = super::Solution::remove_element(&mut nums, val);
        assert_eq!(expected_nums.len(), k as usize);
        let mut clean_nums = nums[0..k as usize].to_vec();
        clean_nums.sort();
        expected_nums.sort();

        assert_eq!(clean_nums, expected_nums);

        // tc 4
        let mut nums = vec![3];
        let val = 3;
        let mut expected_nums = vec![];

        let k = super::Solution::remove_element(&mut nums, val);
        assert_eq!(expected_nums.len(), k as usize);
        let mut clean_nums = nums[0..k as usize].to_vec();
        clean_nums.sort();
        expected_nums.sort();

        assert_eq!(clean_nums, expected_nums);
    }
}
