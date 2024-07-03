struct Solution;
// Problem: https://leetcode.com/problems/remove-duplicates-from-sorted-array

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unique_ptr = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[unique_ptr] = nums[i];
                unique_ptr += 1;
            }
        }
        return unique_ptr as i32;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        println!("result nums: {:?}", nums);
        let expected_k = 5;
        let k = super::Solution::remove_duplicates(&mut nums);
        assert_eq!(expected_k, k);

        let mut nums = vec![1, 1, 2];
        let expected_k = 2;
        let k = super::Solution::remove_duplicates(&mut nums);
        println!("result nums: {:?}", nums);
        assert_eq!(expected_k, k);
    }
}
